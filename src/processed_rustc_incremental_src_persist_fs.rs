/* FP:fs.rs-0001 */ // This module manages how the incremental compilation cache is represented in
/* FP:fs.rs-0002 */ // the file system.
/* FP:fs.rs-0003 */ //
/* FP:fs.rs-0004 */ // Incremental compilation caches are managed according to a copy-on-write
/* FP:fs.rs-0005 */ // strategy: Once a complete, consistent cache version is finalized, it is
/* FP:fs.rs-0006 */ // never modified. Instead, when a subsequent compilation session is started,
/* FP:fs.rs-0007 */ // the compiler will allocate a new version of the cache that starts out as
/* FP:fs.rs-0008 */ // a copy of the previous version. Then only this new copy is modified and it
/* FP:fs.rs-0009 */ // will not be visible to other processes until it is finalized. This ensures
/* FP:fs.rs-0010 */ // that multiple compiler processes can be executed concurrently for the same
/* FP:fs.rs-0011 */ // crate without interfering with each other or blocking each other.
/* FP:fs.rs-0012 */ //
/* FP:fs.rs-0013 */ // More concretely this is implemented via the following protocol:
/* FP:fs.rs-0014 */ //
/* FP:fs.rs-0015 */ // 1. For a newly started compilation session, the compiler allocates a
/* FP:fs.rs-0016 */ //    new `session` directory within the incremental compilation directory.
/* FP:fs.rs-0017 */ //    This session directory will have a unique name that ends with the suffix
/* FP:fs.rs-0018 */ //    "-working" and that contains a creation timestamp.
/* FP:fs.rs-0019 */ // 2. Next, the compiler looks for the newest finalized session directory,
/* FP:fs.rs-0020 */ //    that is, a session directory from a previous compilation session that
/* FP:fs.rs-0021 */ //    has been marked as valid and consistent. A session directory is
/* FP:fs.rs-0022 */ //    considered finalized if the "-working" suffix in the directory name has
/* FP:fs.rs-0023 */ //    been replaced by the SVH of the crate.
/* FP:fs.rs-0024 */ // 3. Once the compiler has found a valid, finalized session directory, it will
/* FP:fs.rs-0025 */ //    hard-link/copy its contents into the new "-working" directory. If all
/* FP:fs.rs-0026 */ //    goes well, it will have its own, private copy of the source directory and
/* FP:fs.rs-0027 */ //    subsequently not have to worry about synchronizing with other compiler
/* FP:fs.rs-0028 */ //    processes.
/* FP:fs.rs-0029 */ // 4. Now the compiler can do its normal compilation process, which involves
/* FP:fs.rs-0030 */ //    reading and updating its private session directory.
/* FP:fs.rs-0031 */ // 5. When compilation finishes without errors, the private session directory
/* FP:fs.rs-0032 */ //    will be in a state where it can be used as input for other compilation
/* FP:fs.rs-0033 */ //    sessions. That is, it will contain a dependency graph and cache artifacts
/* FP:fs.rs-0034 */ //    that are consistent with the state of the source code it was compiled
/* FP:fs.rs-0035 */ //    from, with no need to change them ever again. At this point, the compiler
/* FP:fs.rs-0036 */ //    finalizes and "publishes" its private session directory by renaming it
/* FP:fs.rs-0037 */ //    from "s-{timestamp}-{random}-working" to "s-{timestamp}-{SVH}".
/* FP:fs.rs-0038 */ // 6. At this point the "old" session directory that we copied our data from
/* FP:fs.rs-0039 */ //    at the beginning of the session has become obsolete because we have just
/* FP:fs.rs-0040 */ //    published a more current version. Thus the compiler will delete it.
/* FP:fs.rs-0041 */ //
/* FP:fs.rs-0042 */ // ## Garbage Collection
/* FP:fs.rs-0043 */ //
/* FP:fs.rs-0044 */ // Naively following the above protocol might lead to old session directories
/* FP:fs.rs-0045 */ // piling up if a compiler instance crashes for some reason before its able to
/* FP:fs.rs-0046 */ // remove its private session directory. In order to avoid wasting disk space,
/* FP:fs.rs-0047 */ // the compiler also does some garbage collection each time it is started in
/* FP:fs.rs-0048 */ // incremental compilation mode. Specifically, it will scan the incremental
/* FP:fs.rs-0049 */ // compilation directory for private session directories that are not in use
/* FP:fs.rs-0050 */ // any more and will delete those. It will also delete any finalized session
/* FP:fs.rs-0051 */ // directories for a given crate except for the most recent one.
/* FP:fs.rs-0052 */ //
/* FP:fs.rs-0053 */ // ## Synchronization
/* FP:fs.rs-0054 */ //
/* FP:fs.rs-0055 */ // There is some synchronization needed in order for the compiler to be able to
/* FP:fs.rs-0056 */ // determine whether a given private session directory is not in use any more.
/* FP:fs.rs-0057 */ // This is done by creating a lock file for each session directory and
/* FP:fs.rs-0058 */ // locking it while the directory is still being used. Since file locks have
/* FP:fs.rs-0059 */ // operating system support, we can rely on the lock being released if the
/* FP:fs.rs-0060 */ // compiler process dies for some unexpected reason. Thus, when garbage
/* FP:fs.rs-0061 */ // collecting private session directories, the collecting process can determine
/* FP:fs.rs-0062 */ // whether the directory is still in use by trying to acquire a lock on the
/* FP:fs.rs-0063 */ // file. If locking the file fails, the original process must still be alive.
/* FP:fs.rs-0064 */ // If locking the file succeeds, we know that the owning process is not alive
/* FP:fs.rs-0065 */ // any more and we can safely delete the directory.
/* FP:fs.rs-0066 */ // There is still a small time window between the original process creating the
/* FP:fs.rs-0067 */ // lock file and actually locking it. In order to minimize the chance that
/* FP:fs.rs-0068 */ // another process tries to acquire the lock in just that instance, only
/* FP:fs.rs-0069 */ // session directories that are older than a few seconds are considered for
/* FP:fs.rs-0070 */ // garbage collection.
/* FP:fs.rs-0071 */ //
/* FP:fs.rs-0072 */ // Another case that has to be considered is what happens if one process
/* FP:fs.rs-0073 */ // deletes a finalized session directory that another process is currently
/* FP:fs.rs-0074 */ // trying to copy from. This case is also handled via the lock file. Before
/* FP:fs.rs-0075 */ // a process starts copying a finalized session directory, it will acquire a
/* FP:fs.rs-0076 */ // shared lock on the directory's lock file. Any garbage collecting process,
/* FP:fs.rs-0077 */ // on the other hand, will acquire an exclusive lock on the lock file.
/* FP:fs.rs-0078 */ // Thus, if a directory is being collected, any reader process will fail
/* FP:fs.rs-0079 */ // acquiring the shared lock and will leave the directory alone. Conversely,
/* FP:fs.rs-0080 */ // if a collecting process can't acquire the exclusive lock because the
/* FP:fs.rs-0081 */ // directory is currently being read from, it will leave collecting that
/* FP:fs.rs-0082 */ // directory to another process at a later point in time.
/* FP:fs.rs-0083 */ // The exact same scheme is also used when reading the metadata hashes file
/* FP:fs.rs-0084 */ // from an extern crate. When a crate is compiled, the hash values of its
/* FP:fs.rs-0085 */ // metadata are stored in a file in its session directory. When the
/* FP:fs.rs-0086 */ // compilation session of another crate imports the first crate's metadata,
/* FP:fs.rs-0087 */ // it also has to read in the accompanying metadata hashes. It thus will access
/* FP:fs.rs-0088 */ // the finalized session directory of all crates it links to and while doing
/* FP:fs.rs-0089 */ // so, it will also place a read lock on that the respective session directory
/* FP:fs.rs-0090 */ // so that it won't be deleted while the metadata hashes are loaded.
/* FP:fs.rs-0091 */ //
/* FP:fs.rs-0092 */ // ## Preconditions
/* FP:fs.rs-0093 */ //
/* FP:fs.rs-0094 */ // This system relies on two features being available in the file system in
/* FP:fs.rs-0095 */ // order to work really well: file locking and hard linking.
/* FP:fs.rs-0096 */ // If hard linking is not available (like on FAT) the data in the cache
/* FP:fs.rs-0097 */ // actually has to be copied at the beginning of each session.
/* FP:fs.rs-0098 */ // If file locking does not work reliably (like on NFS), some of the
/* FP:fs.rs-0099 */ // synchronization will go haywire.
/* FP:fs.rs-0100 */ // In both cases we recommend to locate the incremental compilation directory
/* FP:fs.rs-0101 */ // on a file system that supports these things.
/* FP:fs.rs-0102 */ // It might be a good idea though to try and detect whether we are on an
/* FP:fs.rs-0103 */ // unsupported file system and emit a warning in that case. This is not yet
/* FP:fs.rs-0104 */ // implemented.
/* FP:fs.rs-0105 */ 
/* FP:fs.rs-0106 */ use std::fs as std_fs;
/* FP:fs.rs-0107 */ use std::io::{self, ErrorKind};
/* FP:fs.rs-0108 */ use std::path::{Path, PathBuf};
/* FP:fs.rs-0109 */ use std::time::{Duration, SystemTime, UNIX_EPOCH};
/* FP:fs.rs-0110 */ 
/* FP:fs.rs-0111 */ use rand::{RngCore, rng};
/* FP:fs.rs-0112 */ use crate::rustc_data_structures::base_n::{BaseNString, CASE_INSENSITIVE, ToBaseN};
/* FP:fs.rs-0113 */ use crate::rustc_data_structures::fx::{FxHashSet, FxIndexSet};
/* FP:fs.rs-0114 */ use crate::rustc_data_structures::svh::Svh;
/* FP:fs.rs-0115 */ use crate::rustc_data_structures::unord::{UnordMap, UnordSet};
/* FP:fs.rs-0116 */ use crate::rustc_data_structures::{base_n, flock};
/* FP:fs.rs-0117 */ use rustc_fs_util::{LinkOrCopy, link_or_copy, try_canonicalize};
/* FP:fs.rs-0118 */ use crate::rustc_complete::bug;
/* FP:fs.rs-0119 */ use crate::rustc_complete::config::CrateType;
/* FP:fs.rs-0120 */ use crate::rustc_complete::output::collect_crate_types;
/* FP:fs.rs-0121 */ use crate::rustc_complete::{Session, StableCrateId};
/* FP:fs.rs-0122 */ use crate::rustc_complete::Symbol;
/* FP:fs.rs-0123 */ use tracing::debug;
/* FP:fs.rs-0124 */ 
/* FP:fs.rs-0125 */ use crate::errors;
/* FP:fs.rs-0126 */ 
/* FP:fs.rs-0127 */ #[cfg(test)]
/* FP:fs.rs-0129 */ 
/* FP:fs.rs-0130 */ const LOCK_FILE_EXT: &str = ".lock";
/* FP:fs.rs-0131 */ const DEP_GRAPH_FILENAME: &str = "dep-graph.bin";
/* FP:fs.rs-0132 */ const STAGING_DEP_GRAPH_FILENAME: &str = "dep-graph.part.bin";
/* FP:fs.rs-0133 */ const WORK_PRODUCTS_FILENAME: &str = "work-products.bin";
/* FP:fs.rs-0134 */ const QUERY_CACHE_FILENAME: &str = "query-cache.bin";
/* FP:fs.rs-0135 */ 
/* FP:fs.rs-0136 */ // We encode integers using the following base, so they are shorter than decimal
/* FP:fs.rs-0137 */ // or hexadecimal numbers (we want short file and directory names). Since these
/* FP:fs.rs-0138 */ // numbers will be used in file names, we choose an encoding that is not
/* FP:fs.rs-0139 */ // case-sensitive (as opposed to base64, for example).
/* FP:fs.rs-0140 */ const INT_ENCODE_BASE: usize = base_n::CASE_INSENSITIVE;
/* FP:fs.rs-0141 */ 
/* FP:fs.rs-0142 */ /// Returns the path to a session's dependency graph.
/* FP:fs.rs-0143 */ pub(crate) fn dep_graph_path(sess: &Session) -> PathBuf {
/* FP:fs.rs-0144 */     in_incr_comp_dir_sess(sess, DEP_GRAPH_FILENAME)
/* FP:fs.rs-0145 */ }
/* FP:fs.rs-0146 */ 
/* FP:fs.rs-0147 */ /// Returns the path to a session's staging dependency graph.
/* FP:fs.rs-0148 */ ///
/* FP:fs.rs-0149 */ /// On the difference between dep-graph and staging dep-graph,
/* FP:fs.rs-0150 */ /// see `build_dep_graph`.
/* FP:fs.rs-0151 */ pub(crate) fn staging_dep_graph_path(sess: &Session) -> PathBuf {
/* FP:fs.rs-0152 */     in_incr_comp_dir_sess(sess, STAGING_DEP_GRAPH_FILENAME)
/* FP:fs.rs-0153 */ }
/* FP:fs.rs-0154 */ 
/* FP:fs.rs-0155 */ pub(crate) fn work_products_path(sess: &Session) -> PathBuf {
/* FP:fs.rs-0156 */     in_incr_comp_dir_sess(sess, WORK_PRODUCTS_FILENAME)
/* FP:fs.rs-0157 */ }
/* FP:fs.rs-0158 */ 
/* FP:fs.rs-0159 */ /// Returns the path to a session's query cache.
/* FP:fs.rs-0160 */ pub(crate) fn query_cache_path(sess: &Session) -> PathBuf {
/* FP:fs.rs-0161 */     in_incr_comp_dir_sess(sess, QUERY_CACHE_FILENAME)
/* FP:fs.rs-0162 */ }
/* FP:fs.rs-0163 */ 
/* FP:fs.rs-0164 */ /// Locks a given session directory.
/* FP:fs.rs-0165 */ fn lock_file_path(session_dir: &Path) -> PathBuf {
/* FP:fs.rs-0166 */     let crate_dir = session_dir.parent().unwrap();
/* FP:fs.rs-0167 */ 
/* FP:fs.rs-0168 */     let directory_name = session_dir
/* FP:fs.rs-0169 */         .file_name()
/* FP:fs.rs-0170 */         .unwrap()
/* FP:fs.rs-0171 */         .to_str()
/* FP:fs.rs-0172 */         .expect("malformed session dir name: contains non-Unicode characters");
/* FP:fs.rs-0173 */ 
/* FP:fs.rs-0174 */     let dash_indices: Vec<_> = directory_name.match_indices('-').map(|(idx, _)| idx).collect();
/* FP:fs.rs-0175 */     if dash_indices.len() != 3 {
/* FP:fs.rs-0176 */         bug!(
/* FP:fs.rs-0177 */             "Encountered incremental compilation session directory with \
/* FP:fs.rs-0178 */               malformed name: {}",
/* FP:fs.rs-0179 */             session_dir.display()
/* FP:fs.rs-0180 */         )
/* FP:fs.rs-0181 */     }
/* FP:fs.rs-0182 */ 
/* FP:fs.rs-0183 */     crate_dir.join(&directory_name[0..dash_indices[2]]).with_extension(&LOCK_FILE_EXT[1..])
/* FP:fs.rs-0184 */ }
/* FP:fs.rs-0185 */ 
/* FP:fs.rs-0186 */ /// Returns the path for a given filename within the incremental compilation directory
/* FP:fs.rs-0187 */ /// in the current session.
/* FP:fs.rs-0188 */ pub fn in_incr_comp_dir_sess(sess: &Session, file_name: &str) -> PathBuf {
/* FP:fs.rs-0189 */     in_incr_comp_dir(&sess.incr_comp_session_dir(), file_name)
/* FP:fs.rs-0190 */ }
/* FP:fs.rs-0191 */ 
/* FP:fs.rs-0192 */ /// Returns the path for a given filename within the incremental compilation directory,
/* FP:fs.rs-0193 */ /// not necessarily from the current session.
/* FP:fs.rs-0194 */ ///
/* FP:fs.rs-0195 */ /// To ensure the file is part of the current session, use [`in_incr_comp_dir_sess`].
/* FP:fs.rs-0196 */ pub fn in_incr_comp_dir(incr_comp_session_dir: &Path, file_name: &str) -> PathBuf {
/* FP:fs.rs-0197 */     incr_comp_session_dir.join(file_name)
/* FP:fs.rs-0198 */ }
/* FP:fs.rs-0199 */ 
/* FP:fs.rs-0200 */ /// Allocates the private session directory.
/* FP:fs.rs-0201 */ ///
/* FP:fs.rs-0202 */ /// If the result of this function is `Ok`, we have a valid incremental
/* FP:fs.rs-0203 */ /// compilation session directory. A valid session
/* FP:fs.rs-0204 */ /// directory is one that contains a locked lock file. It may or may not contain
/* FP:fs.rs-0205 */ /// a dep-graph and work products from a previous session.
/* FP:fs.rs-0206 */ ///
/* FP:fs.rs-0207 */ /// This always attempts to load a dep-graph from the directory.
/* FP:fs.rs-0208 */ /// If loading fails for some reason, we fallback to a disabled `DepGraph`.
/* FP:fs.rs-0209 */ /// See [`rustc_interface::queries::dep_graph`].
/* FP:fs.rs-0210 */ ///
/* FP:fs.rs-0211 */ /// If this function returns an error, it may leave behind an invalid session directory.
/* FP:fs.rs-0212 */ /// The garbage collection will take care of it.
/* FP:fs.rs-0213 */ ///
/* FP:fs.rs-0214 */ /// [`rustc_interface::queries::dep_graph`]: ../../rustc_interface/struct.Queries.html#structfield.dep_graph
/* FP:fs.rs-0215 */ pub(crate) fn prepare_session_directory(sess: &Session, crate_name: Symbol) {
/* FP:fs.rs-0216 */     if sess.opts.incremental.is_none() {
/* FP:fs.rs-0217 */         return;
/* FP:fs.rs-0218 */     }
/* FP:fs.rs-0219 */ 
/* FP:fs.rs-0220 */     let _timer = sess.timer("incr_comp_prepare_session_directory");
/* FP:fs.rs-0221 */ 
/* FP:fs.rs-0222 */     debug!("prepare_session_directory");
/* FP:fs.rs-0223 */ 
/* FP:fs.rs-0224 */     // {incr-comp-dir}/{crate-name-and-disambiguator}
/* FP:fs.rs-0225 */     let crate_dir = crate_path(sess, crate_name);
/* FP:fs.rs-0226 */     debug!("crate-dir: {}", crate_dir.display());
/* FP:fs.rs-0227 */     create_dir(sess, &crate_dir, "crate");
/* FP:fs.rs-0228 */ 
/* FP:fs.rs-0229 */     // Hack: canonicalize the path *after creating the directory*
/* FP:fs.rs-0230 */     // because, on windows, long paths can cause problems;
/* FP:fs.rs-0231 */     // canonicalization inserts this weird prefix that makes windows
/* FP:fs.rs-0232 */     // tolerate long paths.
/* FP:fs.rs-0233 */     let crate_dir = match try_canonicalize(&crate_dir) {
/* FP:fs.rs-0234 */         Ok(v) => v,
/* FP:fs.rs-0235 */         Err(err) => {
/* FP:fs.rs-0236 */             sess.dcx().emit_fatal(errors::CanonicalizePath { path: crate_dir, err });
/* FP:fs.rs-0237 */         }
/* FP:fs.rs-0238 */     };
/* FP:fs.rs-0239 */ 
/* FP:fs.rs-0240 */     let mut source_directories_already_tried = FxHashSet::default();
/* FP:fs.rs-0241 */ 
/* FP:fs.rs-0242 */     loop {
/* FP:fs.rs-0243 */         // Generate a session directory of the form:
/* FP:fs.rs-0244 */         //
/* FP:fs.rs-0245 */         // {incr-comp-dir}/{crate-name-and-disambiguator}/s-{timestamp}-{random}-working
/* FP:fs.rs-0246 */         let session_dir = generate_session_dir_path(&crate_dir);
/* FP:fs.rs-0247 */         debug!("session-dir: {}", session_dir.display());
/* FP:fs.rs-0248 */ 
/* FP:fs.rs-0249 */         // Lock the new session directory. If this fails, return an
/* FP:fs.rs-0250 */         // error without retrying
/* FP:fs.rs-0251 */         let (directory_lock, lock_file_path) = lock_directory(sess, &session_dir);
/* FP:fs.rs-0252 */ 
/* FP:fs.rs-0253 */         // Now that we have the lock, we can actually create the session
/* FP:fs.rs-0254 */         // directory
/* FP:fs.rs-0255 */         create_dir(sess, &session_dir, "session");
/* FP:fs.rs-0256 */ 
/* FP:fs.rs-0257 */         // Find a suitable source directory to copy from. Ignore those that we
/* FP:fs.rs-0258 */         // have already tried before.
/* FP:fs.rs-0259 */         let source_directory = find_source_directory(&crate_dir, &source_directories_already_tried);
/* FP:fs.rs-0260 */ 
/* FP:fs.rs-0261 */         let Some(source_directory) = source_directory else {
/* FP:fs.rs-0262 */             // There's nowhere to copy from, we're done
/* FP:fs.rs-0263 */             debug!(
/* FP:fs.rs-0264 */                 "no source directory found. Continuing with empty session \
/* FP:fs.rs-0265 */                     directory."
/* FP:fs.rs-0266 */             );
/* FP:fs.rs-0267 */ 
/* FP:fs.rs-0268 */             sess.init_incr_comp_session(session_dir, directory_lock);
/* FP:fs.rs-0269 */             return;
/* FP:fs.rs-0270 */         };
/* FP:fs.rs-0271 */ 
/* FP:fs.rs-0272 */         debug!("attempting to copy data from source: {}", source_directory.display());
/* FP:fs.rs-0273 */ 
/* FP:fs.rs-0274 */         // Try copying over all files from the source directory
/* FP:fs.rs-0275 */         if let Ok(allows_links) = copy_files(sess, &session_dir, &source_directory) {
/* FP:fs.rs-0276 */             debug!("successfully copied data from: {}", source_directory.display());
/* FP:fs.rs-0277 */ 
/* FP:fs.rs-0278 */             if !allows_links {
/* FP:fs.rs-0279 */                 sess.dcx().emit_warn(errors::HardLinkFailed { path: &session_dir });
/* FP:fs.rs-0280 */             }
/* FP:fs.rs-0281 */ 
/* FP:fs.rs-0282 */             sess.init_incr_comp_session(session_dir, directory_lock);
/* FP:fs.rs-0283 */             return;
/* FP:fs.rs-0284 */         } else {
/* FP:fs.rs-0285 */             debug!("copying failed - trying next directory");
/* FP:fs.rs-0286 */ 
/* FP:fs.rs-0287 */             // Something went wrong while trying to copy/link files from the
/* FP:fs.rs-0288 */             // source directory. Try again with a different one.
/* FP:fs.rs-0289 */             source_directories_already_tried.insert(source_directory);
/* FP:fs.rs-0290 */ 
/* FP:fs.rs-0291 */             // Try to remove the session directory we just allocated. We don't
/* FP:fs.rs-0292 */             // know if there's any garbage in it from the failed copy action.
/* FP:fs.rs-0293 */             if let Err(err) = std_fs::remove_dir_all(&session_dir) {
/* FP:fs.rs-0294 */                 sess.dcx().emit_warn(errors::DeletePartial { path: &session_dir, err });
/* FP:fs.rs-0295 */             }
/* FP:fs.rs-0296 */ 
/* FP:fs.rs-0297 */             delete_session_dir_lock_file(sess, &lock_file_path);
/* FP:fs.rs-0298 */             drop(directory_lock);
/* FP:fs.rs-0299 */         }
/* FP:fs.rs-0300 */     }
/* FP:fs.rs-0301 */ }
/* FP:fs.rs-0302 */ 
/* FP:fs.rs-0303 */ /// This function finalizes and thus 'publishes' the session directory by
/* FP:fs.rs-0304 */ /// renaming it to `s-{timestamp}-{svh}` and releasing the file lock.
/* FP:fs.rs-0305 */ /// If there have been compilation errors, however, this function will just
/* FP:fs.rs-0306 */ /// delete the presumably invalid session directory.
/* FP:fs.rs-0307 */ pub fn finalize_session_directory(sess: &Session, svh: Option<Svh>) {
/* FP:fs.rs-0308 */     if sess.opts.incremental.is_none() {
/* FP:fs.rs-0309 */         return;
/* FP:fs.rs-0310 */     }
/* FP:fs.rs-0311 */     // The svh is always produced when incr. comp. is enabled.
/* FP:fs.rs-0312 */     let svh = svh.unwrap();
/* FP:fs.rs-0313 */ 
/* FP:fs.rs-0314 */     let _timer = sess.timer("incr_comp_finalize_session_directory");
/* FP:fs.rs-0315 */ 
/* FP:fs.rs-0316 */     let incr_comp_session_dir: PathBuf = sess.incr_comp_session_dir().clone();
/* FP:fs.rs-0317 */ 
/* FP:fs.rs-0318 */     if sess.dcx().has_errors_or_delayed_bugs().is_some() {
/* FP:fs.rs-0319 */         // If there have been any errors during compilation, we don't want to
/* FP:fs.rs-0320 */         // publish this session directory. Rather, we'll just delete it.
/* FP:fs.rs-0321 */ 
/* FP:fs.rs-0322 */         debug!(
/* FP:fs.rs-0323 */             "finalize_session_directory() - invalidating session directory: {}",
/* FP:fs.rs-0324 */             incr_comp_session_dir.display()
/* FP:fs.rs-0325 */         );
/* FP:fs.rs-0326 */ 
/* FP:fs.rs-0327 */         if let Err(err) = std_fs::remove_dir_all(&*incr_comp_session_dir) {
/* FP:fs.rs-0328 */             sess.dcx().emit_warn(errors::DeleteFull { path: &incr_comp_session_dir, err });
/* FP:fs.rs-0329 */         }
/* FP:fs.rs-0330 */ 
/* FP:fs.rs-0331 */         let lock_file_path = lock_file_path(&*incr_comp_session_dir);
/* FP:fs.rs-0332 */         delete_session_dir_lock_file(sess, &lock_file_path);
/* FP:fs.rs-0333 */         sess.mark_incr_comp_session_as_invalid();
/* FP:fs.rs-0334 */     }
/* FP:fs.rs-0335 */ 
/* FP:fs.rs-0336 */     debug!("finalize_session_directory() - session directory: {}", incr_comp_session_dir.display());
/* FP:fs.rs-0337 */ 
/* FP:fs.rs-0338 */     let mut sub_dir_name = incr_comp_session_dir
/* FP:fs.rs-0339 */         .file_name()
/* FP:fs.rs-0340 */         .unwrap()
/* FP:fs.rs-0341 */         .to_str()
/* FP:fs.rs-0342 */         .expect("malformed session dir name: contains non-Unicode characters")
/* FP:fs.rs-0343 */         .to_string();
/* FP:fs.rs-0344 */ 
/* FP:fs.rs-0345 */     // Keep the 's-{timestamp}-{random-number}' prefix, but replace "working" with the SVH of the crate
/* FP:fs.rs-0346 */     sub_dir_name.truncate(sub_dir_name.len() - "working".len());
/* FP:fs.rs-0347 */     // Double-check that we kept this: "s-{timestamp}-{random-number}-"
/* FP:fs.rs-0348 */     assert!(sub_dir_name.ends_with('-'), "{:?}", sub_dir_name);
/* FP:fs.rs-0349 */     assert!(sub_dir_name.as_bytes().iter().filter(|b| **b == b'-').count() == 3);
/* FP:fs.rs-0350 */ 
/* FP:fs.rs-0351 */     // Append the SVH
/* FP:fs.rs-0352 */     sub_dir_name.push_str(&svh.as_u128().to_base_fixed_len(CASE_INSENSITIVE));
/* FP:fs.rs-0353 */ 
/* FP:fs.rs-0354 */     // Create the full path
/* FP:fs.rs-0355 */     let new_path = incr_comp_session_dir.parent().unwrap().join(&*sub_dir_name);
/* FP:fs.rs-0356 */     debug!("finalize_session_directory() - new path: {}", new_path.display());
/* FP:fs.rs-0357 */ 
/* FP:fs.rs-0358 */     match rename_path_with_retry(&*incr_comp_session_dir, &new_path, 3) {
/* FP:fs.rs-0359 */         Ok(_) => {
/* FP:fs.rs-0360 */             debug!("finalize_session_directory() - directory renamed successfully");
/* FP:fs.rs-0361 */ 
/* FP:fs.rs-0362 */             // This unlocks the directory
/* FP:fs.rs-0363 */             sess.finalize_incr_comp_session(new_path);
/* FP:fs.rs-0364 */         }
/* FP:fs.rs-0365 */         Err(e) => {
/* FP:fs.rs-0366 */             // Warn about the error. However, no need to abort compilation now.
/* FP:fs.rs-0367 */             sess.dcx().emit_warn(errors::Finalize { path: &incr_comp_session_dir, err: e });
/* FP:fs.rs-0368 */ 
/* FP:fs.rs-0369 */             debug!("finalize_session_directory() - error, marking as invalid");
/* FP:fs.rs-0370 */             // Drop the file lock, so we can garage collect
/* FP:fs.rs-0371 */             sess.mark_incr_comp_session_as_invalid();
/* FP:fs.rs-0372 */         }
/* FP:fs.rs-0373 */     }
/* FP:fs.rs-0374 */ 
/* FP:fs.rs-0375 */     let _ = garbage_collect_session_directories(sess);
/* FP:fs.rs-0376 */ }
/* FP:fs.rs-0377 */ 
/* FP:fs.rs-0378 */ pub(crate) fn delete_all_session_dir_contents(sess: &Session) -> io::Result<()> {
/* FP:fs.rs-0379 */     let sess_dir_iterator = sess.incr_comp_session_dir().read_dir()?;
/* FP:fs.rs-0380 */     for entry in sess_dir_iterator {
/* FP:fs.rs-0381 */         let entry = entry?;
/* FP:fs.rs-0382 */         safe_remove_file(&entry.path())?
/* FP:fs.rs-0383 */     }
/* FP:fs.rs-0384 */     Ok(())
/* FP:fs.rs-0385 */ }
/* FP:fs.rs-0386 */ 
/* FP:fs.rs-0387 */ fn copy_files(sess: &Session, target_dir: &Path, source_dir: &Path) -> Result<bool, ()> {
/* FP:fs.rs-0388 */     // We acquire a shared lock on the lock file of the directory, so that
/* FP:fs.rs-0389 */     // nobody deletes it out from under us while we are reading from it.
/* FP:fs.rs-0390 */     let lock_file_path = lock_file_path(source_dir);
/* FP:fs.rs-0391 */ 
/* FP:fs.rs-0392 */     // not exclusive
/* FP:fs.rs-0393 */     let Ok(_lock) = flock::Lock::new(
/* FP:fs.rs-0394 */         &lock_file_path,
/* FP:fs.rs-0395 */         false, // don't wait,
/* FP:fs.rs-0396 */         false, // don't create
/* FP:fs.rs-0397 */         false,
/* FP:fs.rs-0398 */     ) else {
/* FP:fs.rs-0399 */         // Could not acquire the lock, don't try to copy from here
/* FP:fs.rs-0400 */         return Err(());
/* FP:fs.rs-0401 */     };
/* FP:fs.rs-0402 */ 
/* FP:fs.rs-0403 */     let Ok(source_dir_iterator) = source_dir.read_dir() else {
/* FP:fs.rs-0404 */         return Err(());
/* FP:fs.rs-0405 */     };
/* FP:fs.rs-0406 */ 
/* FP:fs.rs-0407 */     let mut files_linked = 0;
/* FP:fs.rs-0408 */     let mut files_copied = 0;
/* FP:fs.rs-0409 */ 
/* FP:fs.rs-0410 */     for entry in source_dir_iterator {
/* FP:fs.rs-0411 */         match entry {
/* FP:fs.rs-0412 */             Ok(entry) => {
/* FP:fs.rs-0413 */                 let file_name = entry.file_name();
/* FP:fs.rs-0414 */ 
/* FP:fs.rs-0415 */                 let target_file_path = target_dir.join(file_name);
/* FP:fs.rs-0416 */                 let source_path = entry.path();
/* FP:fs.rs-0417 */ 
/* FP:fs.rs-0418 */                 debug!("copying into session dir: {}", source_path.display());
/* FP:fs.rs-0419 */                 match link_or_copy(source_path, target_file_path) {
/* FP:fs.rs-0420 */                     Ok(LinkOrCopy::Link) => files_linked += 1,
/* FP:fs.rs-0421 */                     Ok(LinkOrCopy::Copy) => files_copied += 1,
/* FP:fs.rs-0422 */                     Err(_) => return Err(()),
/* FP:fs.rs-0423 */                 }
/* FP:fs.rs-0424 */             }
/* FP:fs.rs-0425 */             Err(_) => return Err(()),
/* FP:fs.rs-0426 */         }
/* FP:fs.rs-0427 */     }
/* FP:fs.rs-0428 */ 
/* FP:fs.rs-0429 */     if sess.opts.unstable_opts.incremental_info {
/* FP:fs.rs-0430 */         eprintln!(
/* FP:fs.rs-0431 */             "[incremental] session directory: \
/* FP:fs.rs-0432 */                   {files_linked} files hard-linked"
/* FP:fs.rs-0433 */         );
/* FP:fs.rs-0434 */         eprintln!(
/* FP:fs.rs-0435 */             "[incremental] session directory: \
/* FP:fs.rs-0436 */                  {files_copied} files copied"
/* FP:fs.rs-0437 */         );
/* FP:fs.rs-0438 */     }
/* FP:fs.rs-0439 */ 
/* FP:fs.rs-0440 */     Ok(files_linked > 0 || files_copied == 0)
/* FP:fs.rs-0441 */ }
/* FP:fs.rs-0442 */ 
/* FP:fs.rs-0443 */ /// Generates unique directory path of the form:
/* FP:fs.rs-0444 */ /// {crate_dir}/s-{timestamp}-{random-number}-working
/* FP:fs.rs-0445 */ fn generate_session_dir_path(crate_dir: &Path) -> PathBuf {
/* FP:fs.rs-0446 */     let timestamp = timestamp_to_string(SystemTime::now());
/* FP:fs.rs-0447 */     debug!("generate_session_dir_path: timestamp = {}", timestamp);
/* FP:fs.rs-0448 */     let random_number = rng().next_u32();
/* FP:fs.rs-0449 */     debug!("generate_session_dir_path: random_number = {}", random_number);
/* FP:fs.rs-0450 */ 
/* FP:fs.rs-0451 */     // Chop the first 3 characters off the timestamp. Those 3 bytes will be zero for a while.
/* FP:fs.rs-0452 */     let (zeroes, timestamp) = timestamp.split_at(3);
/* FP:fs.rs-0453 */     assert_eq!(zeroes, "000");
/* FP:fs.rs-0454 */     let directory_name =
/* FP:fs.rs-0455 */         format!("s-{}-{}-working", timestamp, random_number.to_base_fixed_len(CASE_INSENSITIVE));
/* FP:fs.rs-0456 */     debug!("generate_session_dir_path: directory_name = {}", directory_name);
/* FP:fs.rs-0457 */     let directory_path = crate_dir.join(directory_name);
/* FP:fs.rs-0458 */     debug!("generate_session_dir_path: directory_path = {}", directory_path.display());
/* FP:fs.rs-0459 */     directory_path
/* FP:fs.rs-0460 */ }
/* FP:fs.rs-0461 */ 
/* FP:fs.rs-0462 */ fn create_dir(sess: &Session, path: &Path, dir_tag: &str) {
/* FP:fs.rs-0463 */     match std_fs::create_dir_all(path) {
/* FP:fs.rs-0464 */         Ok(()) => {
/* FP:fs.rs-0465 */             debug!("{} directory created successfully", dir_tag);
/* FP:fs.rs-0466 */         }
/* FP:fs.rs-0467 */         Err(err) => sess.dcx().emit_fatal(errors::CreateIncrCompDir { tag: dir_tag, path, err }),
/* FP:fs.rs-0468 */     }
/* FP:fs.rs-0469 */ }
/* FP:fs.rs-0470 */ 
/* FP:fs.rs-0471 */ /// Allocate the lock-file and lock it.
/* FP:fs.rs-0472 */ fn lock_directory(sess: &Session, session_dir: &Path) -> (flock::Lock, PathBuf) {
/* FP:fs.rs-0473 */     let lock_file_path = lock_file_path(session_dir);
/* FP:fs.rs-0474 */     debug!("lock_directory() - lock_file: {}", lock_file_path.display());
/* FP:fs.rs-0475 */ 
/* FP:fs.rs-0476 */     match flock::Lock::new(
/* FP:fs.rs-0477 */         &lock_file_path,
/* FP:fs.rs-0478 */         false, // don't wait
/* FP:fs.rs-0479 */         true,  // create the lock file
/* FP:fs.rs-0480 */         true,
/* FP:fs.rs-0481 */     ) {
/* FP:fs.rs-0482 */         // the lock should be exclusive
/* FP:fs.rs-0483 */         Ok(lock) => (lock, lock_file_path),
/* FP:fs.rs-0484 */         Err(lock_err) => {
/* FP:fs.rs-0485 */             let is_unsupported_lock = flock::Lock::error_unsupported(&lock_err);
/* FP:fs.rs-0486 */             sess.dcx().emit_fatal(errors::CreateLock {
/* FP:fs.rs-0487 */                 lock_err,
/* FP:fs.rs-0488 */                 session_dir,
/* FP:fs.rs-0489 */                 is_unsupported_lock,
/* FP:fs.rs-0490 */                 is_cargo: crate::rustc_session::utils::was_invoked_from_cargo(),
/* FP:fs.rs-0491 */             });
/* FP:fs.rs-0492 */         }
/* FP:fs.rs-0493 */     }
/* FP:fs.rs-0494 */ }
/* FP:fs.rs-0495 */ 
/* FP:fs.rs-0496 */ fn delete_session_dir_lock_file(sess: &Session, lock_file_path: &Path) {
/* FP:fs.rs-0497 */     if let Err(err) = safe_remove_file(lock_file_path) {
/* FP:fs.rs-0498 */         sess.dcx().emit_warn(errors::DeleteLock { path: lock_file_path, err });
/* FP:fs.rs-0499 */     }
/* FP:fs.rs-0500 */ }
/* FP:fs.rs-0501 */ 
/* FP:fs.rs-0502 */ /// Finds the most recent published session directory that is not in the
/* FP:fs.rs-0503 */ /// ignore-list.
/* FP:fs.rs-0504 */ fn find_source_directory(
/* FP:fs.rs-0505 */     crate_dir: &Path,
/* FP:fs.rs-0506 */     source_directories_already_tried: &FxHashSet<PathBuf>,
/* FP:fs.rs-0507 */ ) -> Option<PathBuf> {
/* FP:fs.rs-0508 */     let iter = crate_dir
/* FP:fs.rs-0509 */         .read_dir()
/* FP:fs.rs-0510 */         .unwrap() // FIXME
/* FP:fs.rs-0511 */         .filter_map(|e| e.ok().map(|e| e.path()));
/* FP:fs.rs-0512 */ 
/* FP:fs.rs-0513 */     find_source_directory_in_iter(iter, source_directories_already_tried)
/* FP:fs.rs-0514 */ }
/* FP:fs.rs-0515 */ 
/* FP:fs.rs-0516 */ fn find_source_directory_in_iter<I>(
/* FP:fs.rs-0517 */     iter: I,
/* FP:fs.rs-0518 */     source_directories_already_tried: &FxHashSet<PathBuf>,
/* FP:fs.rs-0519 */ ) -> Option<PathBuf>
/* FP:fs.rs-0520 */ where
/* FP:fs.rs-0521 */     I: Iterator<Item = PathBuf>,
/* FP:fs.rs-0522 */ {
/* FP:fs.rs-0523 */     let mut best_candidate = (UNIX_EPOCH, None);
/* FP:fs.rs-0524 */ 
/* FP:fs.rs-0525 */     for session_dir in iter {
/* FP:fs.rs-0526 */         debug!("find_source_directory_in_iter - inspecting `{}`", session_dir.display());
/* FP:fs.rs-0527 */ 
/* FP:fs.rs-0528 */         let Some(directory_name) = session_dir.file_name().unwrap().to_str() else {
/* FP:fs.rs-0529 */             debug!("find_source_directory_in_iter - ignoring");
/* FP:fs.rs-0530 */             continue;
/* FP:fs.rs-0531 */         };
/* FP:fs.rs-0532 */ 
/* FP:fs.rs-0533 */         if source_directories_already_tried.contains(&session_dir)
/* FP:fs.rs-0534 */             || !is_session_directory(&directory_name)
/* FP:fs.rs-0535 */             || !is_finalized(&directory_name)
/* FP:fs.rs-0536 */         {
/* FP:fs.rs-0537 */             debug!("find_source_directory_in_iter - ignoring");
/* FP:fs.rs-0538 */             continue;
/* FP:fs.rs-0539 */         }
/* FP:fs.rs-0540 */ 
/* FP:fs.rs-0541 */         let timestamp = match extract_timestamp_from_session_dir(&directory_name) {
/* FP:fs.rs-0542 */             Ok(timestamp) => timestamp,
/* FP:fs.rs-0543 */             Err(e) => {
/* FP:fs.rs-0544 */                 debug!("unexpected incr-comp session dir: {}: {}", session_dir.display(), e);
/* FP:fs.rs-0545 */                 continue;
/* FP:fs.rs-0546 */             }
/* FP:fs.rs-0547 */         };
/* FP:fs.rs-0548 */ 
/* FP:fs.rs-0549 */         if timestamp > best_candidate.0 {
/* FP:fs.rs-0550 */             best_candidate = (timestamp, Some(session_dir.clone()));
/* FP:fs.rs-0551 */         }
/* FP:fs.rs-0552 */     }
/* FP:fs.rs-0553 */ 
/* FP:fs.rs-0554 */     best_candidate.1
/* FP:fs.rs-0555 */ }
/* FP:fs.rs-0556 */ 
/* FP:fs.rs-0557 */ fn is_finalized(directory_name: &str) -> bool {
/* FP:fs.rs-0558 */     !directory_name.ends_with("-working")
/* FP:fs.rs-0559 */ }
/* FP:fs.rs-0560 */ 
/* FP:fs.rs-0561 */ fn is_session_directory(directory_name: &str) -> bool {
/* FP:fs.rs-0562 */     directory_name.starts_with("s-") && !directory_name.ends_with(LOCK_FILE_EXT)
/* FP:fs.rs-0563 */ }
/* FP:fs.rs-0564 */ 
/* FP:fs.rs-0565 */ fn is_session_directory_lock_file(file_name: &str) -> bool {
/* FP:fs.rs-0566 */     file_name.starts_with("s-") && file_name.ends_with(LOCK_FILE_EXT)
/* FP:fs.rs-0567 */ }
/* FP:fs.rs-0568 */ 
/* FP:fs.rs-0569 */ fn extract_timestamp_from_session_dir(directory_name: &str) -> Result<SystemTime, &'static str> {
/* FP:fs.rs-0570 */     if !is_session_directory(directory_name) {
/* FP:fs.rs-0571 */         return Err("not a directory");
/* FP:fs.rs-0572 */     }
/* FP:fs.rs-0573 */ 
/* FP:fs.rs-0574 */     let dash_indices: Vec<_> = directory_name.match_indices('-').map(|(idx, _)| idx).collect();
/* FP:fs.rs-0575 */     if dash_indices.len() != 3 {
/* FP:fs.rs-0576 */         return Err("not three dashes in name");
/* FP:fs.rs-0577 */     }
/* FP:fs.rs-0578 */ 
/* FP:fs.rs-0579 */     string_to_timestamp(&directory_name[dash_indices[0] + 1..dash_indices[1]])
/* FP:fs.rs-0580 */ }
/* FP:fs.rs-0581 */ 
/* FP:fs.rs-0582 */ fn timestamp_to_string(timestamp: SystemTime) -> BaseNString {
/* FP:fs.rs-0583 */     let duration = timestamp.duration_since(UNIX_EPOCH).unwrap();
/* FP:fs.rs-0584 */     let micros: u64 = duration.as_micros().try_into().unwrap();
/* FP:fs.rs-0585 */     micros.to_base_fixed_len(CASE_INSENSITIVE)
/* FP:fs.rs-0586 */ }
/* FP:fs.rs-0587 */ 
/* FP:fs.rs-0588 */ fn string_to_timestamp(s: &str) -> Result<SystemTime, &'static str> {
/* FP:fs.rs-0589 */     let micros_since_unix_epoch = match u64::from_str_radix(s, INT_ENCODE_BASE as u32) {
/* FP:fs.rs-0590 */         Ok(micros) => micros,
/* FP:fs.rs-0591 */         Err(_) => return Err("timestamp not an int"),
/* FP:fs.rs-0592 */     };
/* FP:fs.rs-0593 */ 
/* FP:fs.rs-0594 */     let duration = Duration::from_micros(micros_since_unix_epoch);
/* FP:fs.rs-0595 */     Ok(UNIX_EPOCH + duration)
/* FP:fs.rs-0596 */ }
/* FP:fs.rs-0597 */ 
/* FP:fs.rs-0598 */ fn crate_path(sess: &Session, crate_name: Symbol) -> PathBuf {
/* FP:fs.rs-0599 */     let incr_dir = sess.opts.incremental.as_ref().unwrap().clone();
/* FP:fs.rs-0600 */ 
/* FP:fs.rs-0601 */     let crate_types = collect_crate_types(sess, &[]);
/* FP:fs.rs-0602 */     let stable_crate_id = StableCrateId::new(
/* FP:fs.rs-0603 */         crate_name,
/* FP:fs.rs-0604 */         crate_types.contains(&CrateType::Executable),
/* FP:fs.rs-0605 */         sess.opts.cg.metadata.clone(),
/* FP:fs.rs-0606 */         sess.cfg_version,
/* FP:fs.rs-0607 */     );
/* FP:fs.rs-0608 */ 
/* FP:fs.rs-0609 */     let crate_name =
/* FP:fs.rs-0610 */         format!("{crate_name}-{}", stable_crate_id.as_u64().to_base_fixed_len(CASE_INSENSITIVE));
/* FP:fs.rs-0611 */     incr_dir.join(crate_name)
/* FP:fs.rs-0612 */ }
/* FP:fs.rs-0613 */ 
/* FP:fs.rs-0614 */ fn is_old_enough_to_be_collected(timestamp: SystemTime) -> bool {
/* FP:fs.rs-0615 */     timestamp < SystemTime::now() - Duration::from_secs(10)
/* FP:fs.rs-0616 */ }
/* FP:fs.rs-0617 */ 
/* FP:fs.rs-0618 */ /// Runs garbage collection for the current session.
/* FP:fs.rs-0619 */ pub(crate) fn garbage_collect_session_directories(sess: &Session) -> io::Result<()> {
/* FP:fs.rs-0620 */     debug!("garbage_collect_session_directories() - begin");
/* FP:fs.rs-0621 */ 
/* FP:fs.rs-0622 */     let session_directory = sess.incr_comp_session_dir();
/* FP:fs.rs-0623 */     debug!(
/* FP:fs.rs-0624 */         "garbage_collect_session_directories() - session directory: {}",
/* FP:fs.rs-0625 */         session_directory.display()
/* FP:fs.rs-0626 */     );
/* FP:fs.rs-0627 */ 
/* FP:fs.rs-0628 */     let crate_directory = session_directory.parent().unwrap();
/* FP:fs.rs-0629 */     debug!(
/* FP:fs.rs-0630 */         "garbage_collect_session_directories() - crate directory: {}",
/* FP:fs.rs-0631 */         crate_directory.display()
/* FP:fs.rs-0632 */     );
/* FP:fs.rs-0633 */ 
/* FP:fs.rs-0634 */     // First do a pass over the crate directory, collecting lock files and
/* FP:fs.rs-0635 */     // session directories
/* FP:fs.rs-0636 */     let mut session_directories = FxIndexSet::default();
/* FP:fs.rs-0637 */     let mut lock_files = UnordSet::default();
/* FP:fs.rs-0638 */ 
/* FP:fs.rs-0639 */     for dir_entry in crate_directory.read_dir()? {
/* FP:fs.rs-0640 */         let Ok(dir_entry) = dir_entry else {
/* FP:fs.rs-0641 */             // Ignore any errors
/* FP:fs.rs-0642 */             continue;
/* FP:fs.rs-0643 */         };
/* FP:fs.rs-0644 */ 
/* FP:fs.rs-0645 */         let entry_name = dir_entry.file_name();
/* FP:fs.rs-0646 */         let Some(entry_name) = entry_name.to_str() else {
/* FP:fs.rs-0647 */             continue;
/* FP:fs.rs-0648 */         };
/* FP:fs.rs-0649 */ 
/* FP:fs.rs-0650 */         if is_session_directory_lock_file(&entry_name) {
/* FP:fs.rs-0651 */             lock_files.insert(entry_name.to_string());
/* FP:fs.rs-0652 */         } else if is_session_directory(&entry_name) {
/* FP:fs.rs-0653 */             session_directories.insert(entry_name.to_string());
/* FP:fs.rs-0654 */         } else {
/* FP:fs.rs-0655 */             // This is something we don't know, leave it alone
/* FP:fs.rs-0656 */         }
/* FP:fs.rs-0657 */     }
/* FP:fs.rs-0658 */     session_directories.sort();
/* FP:fs.rs-0659 */ 
/* FP:fs.rs-0660 */     // Now map from lock files to session directories
/* FP:fs.rs-0661 */     let lock_file_to_session_dir: UnordMap<String, Option<String>> = lock_files
/* FP:fs.rs-0662 */         .into_items()
/* FP:fs.rs-0663 */         .map(|lock_file_name| {
/* FP:fs.rs-0664 */             assert!(lock_file_name.ends_with(LOCK_FILE_EXT));
/* FP:fs.rs-0665 */             let dir_prefix_end = lock_file_name.len() - LOCK_FILE_EXT.len();
/* FP:fs.rs-0666 */             let session_dir = {
/* FP:fs.rs-0667 */                 let dir_prefix = &lock_file_name[0..dir_prefix_end];
/* FP:fs.rs-0668 */                 session_directories.iter().find(|dir_name| dir_name.starts_with(dir_prefix))
/* FP:fs.rs-0669 */             };
/* FP:fs.rs-0670 */             (lock_file_name, session_dir.map(String::clone))
/* FP:fs.rs-0671 */         })
/* FP:fs.rs-0672 */         .into();
/* FP:fs.rs-0673 */ 
/* FP:fs.rs-0674 */     // Delete all lock files, that don't have an associated directory. They must
/* FP:fs.rs-0675 */     // be some kind of leftover
/* FP:fs.rs-0676 */     for (lock_file_name, directory_name) in
/* FP:fs.rs-0677 */         lock_file_to_session_dir.items().into_sorted_stable_ord()
/* FP:fs.rs-0678 */     {
/* FP:fs.rs-0679 */         if directory_name.is_none() {
/* FP:fs.rs-0680 */             let Ok(timestamp) = extract_timestamp_from_session_dir(lock_file_name) else {
/* FP:fs.rs-0681 */                 debug!(
/* FP:fs.rs-0682 */                     "found lock-file with malformed timestamp: {}",
/* FP:fs.rs-0683 */                     crate_directory.join(&lock_file_name).display()
/* FP:fs.rs-0684 */                 );
/* FP:fs.rs-0685 */                 // Ignore it
/* FP:fs.rs-0686 */                 continue;
/* FP:fs.rs-0687 */             };
/* FP:fs.rs-0688 */ 
/* FP:fs.rs-0689 */             let lock_file_path = crate_directory.join(&*lock_file_name);
/* FP:fs.rs-0690 */ 
/* FP:fs.rs-0691 */             if is_old_enough_to_be_collected(timestamp) {
/* FP:fs.rs-0692 */                 debug!(
/* FP:fs.rs-0693 */                     "garbage_collect_session_directories() - deleting \
/* FP:fs.rs-0694 */                     garbage lock file: {}",
/* FP:fs.rs-0695 */                     lock_file_path.display()
/* FP:fs.rs-0696 */                 );
/* FP:fs.rs-0697 */                 delete_session_dir_lock_file(sess, &lock_file_path);
/* FP:fs.rs-0698 */             } else {
/* FP:fs.rs-0699 */                 debug!(
/* FP:fs.rs-0700 */                     "garbage_collect_session_directories() - lock file with \
/* FP:fs.rs-0701 */                     no session dir not old enough to be collected: {}",
/* FP:fs.rs-0702 */                     lock_file_path.display()
/* FP:fs.rs-0703 */                 );
/* FP:fs.rs-0704 */             }
/* FP:fs.rs-0705 */         }
/* FP:fs.rs-0706 */     }
/* FP:fs.rs-0707 */ 
/* FP:fs.rs-0708 */     // Filter out `None` directories
/* FP:fs.rs-0709 */     let lock_file_to_session_dir: UnordMap<String, String> = lock_file_to_session_dir
/* FP:fs.rs-0710 */         .into_items()
/* FP:fs.rs-0711 */         .filter_map(|(lock_file_name, directory_name)| directory_name.map(|n| (lock_file_name, n)))
/* FP:fs.rs-0712 */         .into();
/* FP:fs.rs-0713 */ 
/* FP:fs.rs-0714 */     // Delete all session directories that don't have a lock file.
/* FP:fs.rs-0715 */     for directory_name in session_directories {
/* FP:fs.rs-0716 */         if !lock_file_to_session_dir.items().any(|(_, dir)| *dir == directory_name) {
/* FP:fs.rs-0717 */             let path = crate_directory.join(directory_name);
/* FP:fs.rs-0718 */             if let Err(err) = std_fs::remove_dir_all(&path) {
/* FP:fs.rs-0719 */                 sess.dcx().emit_warn(errors::InvalidGcFailed { path: &path, err });
/* FP:fs.rs-0720 */             }
/* FP:fs.rs-0721 */         }
/* FP:fs.rs-0722 */     }
/* FP:fs.rs-0723 */ 
/* FP:fs.rs-0724 */     // Now garbage collect the valid session directories.
/* FP:fs.rs-0725 */     let deletion_candidates =
/* FP:fs.rs-0726 */         lock_file_to_session_dir.items().filter_map(|(lock_file_name, directory_name)| {
/* FP:fs.rs-0727 */             debug!("garbage_collect_session_directories() - inspecting: {}", directory_name);
/* FP:fs.rs-0728 */ 
/* FP:fs.rs-0729 */             let Ok(timestamp) = extract_timestamp_from_session_dir(directory_name) else {
/* FP:fs.rs-0730 */                 debug!(
/* FP:fs.rs-0731 */                     "found session-dir with malformed timestamp: {}",
/* FP:fs.rs-0732 */                     crate_directory.join(directory_name).display()
/* FP:fs.rs-0733 */                 );
/* FP:fs.rs-0734 */                 // Ignore it
/* FP:fs.rs-0735 */                 return None;
/* FP:fs.rs-0736 */             };
/* FP:fs.rs-0737 */ 
/* FP:fs.rs-0738 */             if is_finalized(directory_name) {
/* FP:fs.rs-0739 */                 let lock_file_path = crate_directory.join(lock_file_name);
/* FP:fs.rs-0740 */                 match flock::Lock::new(
/* FP:fs.rs-0741 */                     &lock_file_path,
/* FP:fs.rs-0742 */                     false, // don't wait
/* FP:fs.rs-0743 */                     false, // don't create the lock-file
/* FP:fs.rs-0744 */                     true,
/* FP:fs.rs-0745 */                 ) {
/* FP:fs.rs-0746 */                     // get an exclusive lock
/* FP:fs.rs-0747 */                     Ok(lock) => {
/* FP:fs.rs-0748 */                         debug!(
/* FP:fs.rs-0749 */                             "garbage_collect_session_directories() - \
/* FP:fs.rs-0750 */                             successfully acquired lock"
/* FP:fs.rs-0751 */                         );
/* FP:fs.rs-0752 */                         debug!(
/* FP:fs.rs-0753 */                             "garbage_collect_session_directories() - adding \
/* FP:fs.rs-0754 */                             deletion candidate: {}",
/* FP:fs.rs-0755 */                             directory_name
/* FP:fs.rs-0756 */                         );
/* FP:fs.rs-0757 */ 
/* FP:fs.rs-0758 */                         // Note that we are holding on to the lock
/* FP:fs.rs-0759 */                         return Some((
/* FP:fs.rs-0760 */                             (timestamp, crate_directory.join(directory_name)),
/* FP:fs.rs-0761 */                             Some(lock),
/* FP:fs.rs-0762 */                         ));
/* FP:fs.rs-0763 */                     }
/* FP:fs.rs-0764 */                     Err(_) => {
/* FP:fs.rs-0765 */                         debug!(
/* FP:fs.rs-0766 */                             "garbage_collect_session_directories() - \
/* FP:fs.rs-0767 */                             not collecting, still in use"
/* FP:fs.rs-0768 */                         );
/* FP:fs.rs-0769 */                     }
/* FP:fs.rs-0770 */                 }
/* FP:fs.rs-0771 */             } else if is_old_enough_to_be_collected(timestamp) {
/* FP:fs.rs-0772 */                 // When cleaning out "-working" session directories, i.e.
/* FP:fs.rs-0773 */                 // session directories that might still be in use by another
/* FP:fs.rs-0774 */                 // compiler instance, we only look a directories that are
/* FP:fs.rs-0775 */                 // at least ten seconds old. This is supposed to reduce the
/* FP:fs.rs-0776 */                 // chance of deleting a directory in the time window where
/* FP:fs.rs-0777 */                 // the process has allocated the directory but has not yet
/* FP:fs.rs-0778 */                 // acquired the file-lock on it.
/* FP:fs.rs-0779 */ 
/* FP:fs.rs-0780 */                 // Try to acquire the directory lock. If we can't, it
/* FP:fs.rs-0781 */                 // means that the owning process is still alive and we
/* FP:fs.rs-0782 */                 // leave this directory alone.
/* FP:fs.rs-0783 */                 let lock_file_path = crate_directory.join(lock_file_name);
/* FP:fs.rs-0784 */                 match flock::Lock::new(
/* FP:fs.rs-0785 */                     &lock_file_path,
/* FP:fs.rs-0786 */                     false, // don't wait
/* FP:fs.rs-0787 */                     false, // don't create the lock-file
/* FP:fs.rs-0788 */                     true,
/* FP:fs.rs-0789 */                 ) {
/* FP:fs.rs-0790 */                     // get an exclusive lock
/* FP:fs.rs-0791 */                     Ok(lock) => {
/* FP:fs.rs-0792 */                         debug!(
/* FP:fs.rs-0793 */                             "garbage_collect_session_directories() - \
/* FP:fs.rs-0794 */                             successfully acquired lock"
/* FP:fs.rs-0795 */                         );
/* FP:fs.rs-0796 */ 
/* FP:fs.rs-0797 */                         delete_old(sess, &crate_directory.join(directory_name));
/* FP:fs.rs-0798 */ 
/* FP:fs.rs-0799 */                         // Let's make it explicit that the file lock is released at this point,
/* FP:fs.rs-0800 */                         // or rather, that we held on to it until here
/* FP:fs.rs-0801 */                         drop(lock);
/* FP:fs.rs-0802 */                     }
/* FP:fs.rs-0803 */                     Err(_) => {
/* FP:fs.rs-0804 */                         debug!(
/* FP:fs.rs-0805 */                             "garbage_collect_session_directories() - \
/* FP:fs.rs-0806 */                             not collecting, still in use"
/* FP:fs.rs-0807 */                         );
/* FP:fs.rs-0808 */                     }
/* FP:fs.rs-0809 */                 }
/* FP:fs.rs-0810 */             } else {
/* FP:fs.rs-0811 */                 debug!(
/* FP:fs.rs-0812 */                     "garbage_collect_session_directories() - not finalized, not \
/* FP:fs.rs-0813 */                     old enough"
/* FP:fs.rs-0814 */                 );
/* FP:fs.rs-0815 */             }
/* FP:fs.rs-0816 */             None
/* FP:fs.rs-0817 */         });
/* FP:fs.rs-0818 */     let deletion_candidates = deletion_candidates.into();
/* FP:fs.rs-0819 */ 
/* FP:fs.rs-0820 */     // Delete all but the most recent of the candidates
/* FP:fs.rs-0821 */     all_except_most_recent(deletion_candidates).into_items().all(|(path, lock)| {
/* FP:fs.rs-0822 */         debug!("garbage_collect_session_directories() - deleting `{}`", path.display());
/* FP:fs.rs-0823 */ 
/* FP:fs.rs-0824 */         if let Err(err) = std_fs::remove_dir_all(&path) {
/* FP:fs.rs-0825 */             sess.dcx().emit_warn(errors::FinalizedGcFailed { path: &path, err });
/* FP:fs.rs-0826 */         } else {
/* FP:fs.rs-0827 */             delete_session_dir_lock_file(sess, &lock_file_path(&path));
/* FP:fs.rs-0828 */         }
/* FP:fs.rs-0829 */ 
/* FP:fs.rs-0830 */         // Let's make it explicit that the file lock is released at this point,
/* FP:fs.rs-0831 */         // or rather, that we held on to it until here
/* FP:fs.rs-0832 */         drop(lock);
/* FP:fs.rs-0833 */         true
/* FP:fs.rs-0834 */     });
/* FP:fs.rs-0835 */ 
/* FP:fs.rs-0836 */     Ok(())
/* FP:fs.rs-0837 */ }
/* FP:fs.rs-0838 */ 
/* FP:fs.rs-0839 */ fn delete_old(sess: &Session, path: &Path) {
/* FP:fs.rs-0840 */     debug!("garbage_collect_session_directories() - deleting `{}`", path.display());
/* FP:fs.rs-0841 */ 
/* FP:fs.rs-0842 */     if let Err(err) = std_fs::remove_dir_all(path) {
/* FP:fs.rs-0843 */         sess.dcx().emit_warn(errors::SessionGcFailed { path, err });
/* FP:fs.rs-0844 */     } else {
/* FP:fs.rs-0845 */         delete_session_dir_lock_file(sess, &lock_file_path(path));
/* FP:fs.rs-0846 */     }
/* FP:fs.rs-0847 */ }
/* FP:fs.rs-0848 */ 
/* FP:fs.rs-0849 */ fn all_except_most_recent(
/* FP:fs.rs-0850 */     deletion_candidates: UnordMap<(SystemTime, PathBuf), Option<flock::Lock>>,
/* FP:fs.rs-0851 */ ) -> UnordMap<PathBuf, Option<flock::Lock>> {
/* FP:fs.rs-0852 */     let most_recent = deletion_candidates.items().map(|(&(timestamp, _), _)| timestamp).max();
/* FP:fs.rs-0853 */ 
/* FP:fs.rs-0854 */     if let Some(most_recent) = most_recent {
/* FP:fs.rs-0855 */         deletion_candidates
/* FP:fs.rs-0856 */             .into_items()
/* FP:fs.rs-0857 */             .filter(|&((timestamp, _), _)| timestamp != most_recent)
/* FP:fs.rs-0858 */             .map(|((_, path), lock)| (path, lock))
/* FP:fs.rs-0859 */             .collect()
/* FP:fs.rs-0860 */     } else {
/* FP:fs.rs-0861 */         UnordMap::default()
/* FP:fs.rs-0862 */     }
/* FP:fs.rs-0863 */ }
/* FP:fs.rs-0864 */ 
/* FP:fs.rs-0865 */ fn safe_remove_file(p: &Path) -> io::Result<()> {
/* FP:fs.rs-0866 */     match std_fs::remove_file(p) {
/* FP:fs.rs-0867 */         Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
/* FP:fs.rs-0868 */         result => result,
/* FP:fs.rs-0869 */     }
/* FP:fs.rs-0870 */ }
/* FP:fs.rs-0871 */ 
/* FP:fs.rs-0872 */ // On Windows the compiler would sometimes fail to rename the session directory because
/* FP:fs.rs-0873 */ // the OS thought something was still being accessed in it. So we retry a few times to give
/* FP:fs.rs-0874 */ // the OS time to catch up.
/* FP:fs.rs-0875 */ // See https://github.com/rust-lang/rust/issues/86929.
/* FP:fs.rs-0876 */ fn rename_path_with_retry(from: &Path, to: &Path, mut retries_left: usize) -> std::io::Result<()> {
/* FP:fs.rs-0877 */     loop {
/* FP:fs.rs-0878 */         match std_fs::rename(from, to) {
/* FP:fs.rs-0879 */             Ok(()) => return Ok(()),
/* FP:fs.rs-0880 */             Err(e) => {
/* FP:fs.rs-0881 */                 if retries_left > 0 && e.kind() == ErrorKind::PermissionDenied {
/* FP:fs.rs-0882 */                     // Try again after a short waiting period.
/* FP:fs.rs-0883 */                     std::thread::sleep(Duration::from_millis(50));
/* FP:fs.rs-0884 */                     retries_left -= 1;
/* FP:fs.rs-0885 */                 } else {
/* FP:fs.rs-0886 */                     return Err(e);
/* FP:fs.rs-0887 */                 }
/* FP:fs.rs-0888 */             }
/* FP:fs.rs-0889 */         }
/* FP:fs.rs-0890 */     }
/* FP:fs.rs-0891 */ }