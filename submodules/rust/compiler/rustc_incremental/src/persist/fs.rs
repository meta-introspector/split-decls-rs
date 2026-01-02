mkuse!{use std :: fs as std_fs ;}
mkuse!{use std :: io :: { self , ErrorKind } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: time :: { Duration , SystemTime , UNIX_EPOCH } ;}
mkuse!{use rand :: { RngCore , rng } ;}
mkuse!{use rustc_data_structures :: base_n :: { BaseNString , CASE_INSENSITIVE , ToBaseN } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexSet } ;}
mkuse!{use rustc_data_structures :: svh :: Svh ;}
mkuse!{use rustc_data_structures :: unord :: { UnordMap , UnordSet } ;}
mkuse!{use rustc_data_structures :: { base_n , flock } ;}
mkuse!{use rustc_fs_util :: { LinkOrCopy , link_or_copy , try_canonicalize } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_session :: config :: CrateType ;}
mkuse!{use rustc_session :: output :: collect_crate_types ;}
mkuse!{use rustc_session :: { Session , StableCrateId } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: errors ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{const LOCK_FILE_EXT : & str = ".lock" ;}
mkitem!{const DEP_GRAPH_FILENAME : & str = "dep-graph.bin" ;}
mkitem!{const STAGING_DEP_GRAPH_FILENAME : & str = "dep-graph.part.bin" ;}
mkitem!{const WORK_PRODUCTS_FILENAME : & str = "work-products.bin" ;}
mkitem!{const QUERY_CACHE_FILENAME : & str = "query-cache.bin" ;}
mkitem!{const INT_ENCODE_BASE : usize = base_n :: CASE_INSENSITIVE ;}

macro_rules! dep_graph_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dep_graph_path in module {}", module_path!());
    };
}

mkfn!{
    dep_graph_path_introspect!();
    # [doc = " Returns the path to a session's dependency graph."] pub (crate) fn dep_graph_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , DEP_GRAPH_FILENAME) }
}

macro_rules! staging_dep_graph_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function staging_dep_graph_path in module {}", module_path!());
    };
}

mkfn!{
    staging_dep_graph_path_introspect!();
    # [doc = " Returns the path to a session's staging dependency graph."] # [doc = ""] # [doc = " On the difference between dep-graph and staging dep-graph,"] # [doc = " see `build_dep_graph`."] pub (crate) fn staging_dep_graph_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , STAGING_DEP_GRAPH_FILENAME) }
}

macro_rules! work_products_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function work_products_path in module {}", module_path!());
    };
}

mkfn!{
    work_products_path_introspect!();
    pub (crate) fn work_products_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , WORK_PRODUCTS_FILENAME) }
}

macro_rules! query_cache_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function query_cache_path in module {}", module_path!());
    };
}

mkfn!{
    query_cache_path_introspect!();
    # [doc = " Returns the path to a session's query cache."] pub (crate) fn query_cache_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , QUERY_CACHE_FILENAME) }
}

macro_rules! lock_file_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lock_file_path in module {}", module_path!());
    };
}

mkfn!{
    lock_file_path_introspect!();
    # [doc = " Locks a given session directory."] fn lock_file_path (session_dir : & Path) -> PathBuf { let crate_dir = session_dir . parent () . unwrap () ; let directory_name = session_dir . file_name () . unwrap () . to_str () . expect ("malformed session dir name: contains non-Unicode characters") ; let dash_indices : Vec < _ > = directory_name . match_indices ('-') . map (| (idx , _) | idx) . collect () ; if dash_indices . len () != 3 { bug ! ("Encountered incremental compilation session directory with \
              malformed name: {}" , session_dir . display ()) } crate_dir . join (& directory_name [0 .. dash_indices [2]]) . with_extension (& LOCK_FILE_EXT [1 ..]) }
}

macro_rules! in_incr_comp_dir_sess_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function in_incr_comp_dir_sess in module {}", module_path!());
    };
}

mkfn!{
    in_incr_comp_dir_sess_introspect!();
    # [doc = " Returns the path for a given filename within the incremental compilation directory"] # [doc = " in the current session."] pub fn in_incr_comp_dir_sess (sess : & Session , file_name : & str) -> PathBuf { in_incr_comp_dir (& sess . incr_comp_session_dir () , file_name) }
}

macro_rules! in_incr_comp_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function in_incr_comp_dir in module {}", module_path!());
    };
}

mkfn!{
    in_incr_comp_dir_introspect!();
    # [doc = " Returns the path for a given filename within the incremental compilation directory,"] # [doc = " not necessarily from the current session."] # [doc = ""] # [doc = " To ensure the file is part of the current session, use [`in_incr_comp_dir_sess`]."] pub fn in_incr_comp_dir (incr_comp_session_dir : & Path , file_name : & str) -> PathBuf { incr_comp_session_dir . join (file_name) }
}

macro_rules! prepare_session_directory_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_session_directory in module {}", module_path!());
    };
}

mkfn!{
    prepare_session_directory_introspect!();
    # [doc = " Allocates the private session directory."] # [doc = ""] # [doc = " If the result of this function is `Ok`, we have a valid incremental"] # [doc = " compilation session directory. A valid session"] # [doc = " directory is one that contains a locked lock file. It may or may not contain"] # [doc = " a dep-graph and work products from a previous session."] # [doc = ""] # [doc = " This always attempts to load a dep-graph from the directory."] # [doc = " If loading fails for some reason, we fallback to a disabled `DepGraph`."] # [doc = " See [`rustc_interface::queries::dep_graph`]."] # [doc = ""] # [doc = " If this function returns an error, it may leave behind an invalid session directory."] # [doc = " The garbage collection will take care of it."] # [doc = ""] # [doc = " [`rustc_interface::queries::dep_graph`]: ../../rustc_interface/struct.Queries.html#structfield.dep_graph"] pub (crate) fn prepare_session_directory (sess : & Session , crate_name : Symbol) { if sess . opts . incremental . is_none () { return ; } let _timer = sess . timer ("incr_comp_prepare_session_directory") ; debug ! ("prepare_session_directory") ; let crate_dir = crate_path (sess , crate_name) ; debug ! ("crate-dir: {}" , crate_dir . display ()) ; create_dir (sess , & crate_dir , "crate") ; let crate_dir = match try_canonicalize (& crate_dir) { Ok (v) => v , Err (err) => { sess . dcx () . emit_fatal (errors :: CanonicalizePath { path : crate_dir , err }) ; } } ; let mut source_directories_already_tried = FxHashSet :: default () ; loop { let session_dir = generate_session_dir_path (& crate_dir) ; debug ! ("session-dir: {}" , session_dir . display ()) ; let (directory_lock , lock_file_path) = lock_directory (sess , & session_dir) ; create_dir (sess , & session_dir , "session") ; let source_directory = find_source_directory (& crate_dir , & source_directories_already_tried) ; let Some (source_directory) = source_directory else { debug ! ("no source directory found. Continuing with empty session \
                    directory.") ; sess . init_incr_comp_session (session_dir , directory_lock) ; return ; } ; debug ! ("attempting to copy data from source: {}" , source_directory . display ()) ; if let Ok (allows_links) = copy_files (sess , & session_dir , & source_directory) { debug ! ("successfully copied data from: {}" , source_directory . display ()) ; if ! allows_links { sess . dcx () . emit_warn (errors :: HardLinkFailed { path : & session_dir }) ; } sess . init_incr_comp_session (session_dir , directory_lock) ; return ; } else { debug ! ("copying failed - trying next directory") ; source_directories_already_tried . insert (source_directory) ; if let Err (err) = std_fs :: remove_dir_all (& session_dir) { sess . dcx () . emit_warn (errors :: DeletePartial { path : & session_dir , err }) ; } delete_session_dir_lock_file (sess , & lock_file_path) ; drop (directory_lock) ; } } }
}

macro_rules! finalize_session_directory_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function finalize_session_directory in module {}", module_path!());
    };
}

mkfn!{
    finalize_session_directory_introspect!();
    # [doc = " This function finalizes and thus 'publishes' the session directory by"] # [doc = " renaming it to `s-{timestamp}-{svh}` and releasing the file lock."] # [doc = " If there have been compilation errors, however, this function will just"] # [doc = " delete the presumably invalid session directory."] pub fn finalize_session_directory (sess : & Session , svh : Option < Svh >) { if sess . opts . incremental . is_none () { return ; } let svh = svh . unwrap () ; let _timer = sess . timer ("incr_comp_finalize_session_directory") ; let incr_comp_session_dir : PathBuf = sess . incr_comp_session_dir () . clone () ; if sess . dcx () . has_errors_or_delayed_bugs () . is_some () { debug ! ("finalize_session_directory() - invalidating session directory: {}" , incr_comp_session_dir . display ()) ; if let Err (err) = std_fs :: remove_dir_all (& * incr_comp_session_dir) { sess . dcx () . emit_warn (errors :: DeleteFull { path : & incr_comp_session_dir , err }) ; } let lock_file_path = lock_file_path (& * incr_comp_session_dir) ; delete_session_dir_lock_file (sess , & lock_file_path) ; sess . mark_incr_comp_session_as_invalid () ; } debug ! ("finalize_session_directory() - session directory: {}" , incr_comp_session_dir . display ()) ; let mut sub_dir_name = incr_comp_session_dir . file_name () . unwrap () . to_str () . expect ("malformed session dir name: contains non-Unicode characters") . to_string () ; sub_dir_name . truncate (sub_dir_name . len () - "working" . len ()) ; assert ! (sub_dir_name . ends_with ('-') , "{:?}" , sub_dir_name) ; assert ! (sub_dir_name . as_bytes () . iter () . filter (| b | ** b == b'-') . count () == 3) ; sub_dir_name . push_str (& svh . as_u128 () . to_base_fixed_len (CASE_INSENSITIVE)) ; let new_path = incr_comp_session_dir . parent () . unwrap () . join (& * sub_dir_name) ; debug ! ("finalize_session_directory() - new path: {}" , new_path . display ()) ; match rename_path_with_retry (& * incr_comp_session_dir , & new_path , 3) { Ok (_) => { debug ! ("finalize_session_directory() - directory renamed successfully") ; sess . finalize_incr_comp_session (new_path) ; } Err (e) => { sess . dcx () . emit_warn (errors :: Finalize { path : & incr_comp_session_dir , err : e }) ; debug ! ("finalize_session_directory() - error, marking as invalid") ; sess . mark_incr_comp_session_as_invalid () ; } } let _ = garbage_collect_session_directories (sess) ; }
}

macro_rules! delete_all_session_dir_contents_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function delete_all_session_dir_contents in module {}", module_path!());
    };
}

mkfn!{
    delete_all_session_dir_contents_introspect!();
    pub (crate) fn delete_all_session_dir_contents (sess : & Session) -> io :: Result < () > { let sess_dir_iterator = sess . incr_comp_session_dir () . read_dir () ? ; for entry in sess_dir_iterator { let entry = entry ? ; safe_remove_file (& entry . path ()) ? } Ok (()) }
}

macro_rules! copy_files_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_files in module {}", module_path!());
    };
}

mkfn!{
    copy_files_introspect!();
    fn copy_files (sess : & Session , target_dir : & Path , source_dir : & Path) -> Result < bool , () > { let lock_file_path = lock_file_path (source_dir) ; let Ok (_lock) = flock :: Lock :: new (& lock_file_path , false , false , false ,) else { return Err (()) ; } ; let Ok (source_dir_iterator) = source_dir . read_dir () else { return Err (()) ; } ; let mut files_linked = 0 ; let mut files_copied = 0 ; for entry in source_dir_iterator { match entry { Ok (entry) => { let file_name = entry . file_name () ; let target_file_path = target_dir . join (file_name) ; let source_path = entry . path () ; debug ! ("copying into session dir: {}" , source_path . display ()) ; match link_or_copy (source_path , target_file_path) { Ok (LinkOrCopy :: Link) => files_linked += 1 , Ok (LinkOrCopy :: Copy) => files_copied += 1 , Err (_) => return Err (()) , } } Err (_) => return Err (()) , } } if sess . opts . unstable_opts . incremental_info { eprintln ! ("[incremental] session directory: \
                  {files_linked} files hard-linked") ; eprintln ! ("[incremental] session directory: \
                 {files_copied} files copied") ; } Ok (files_linked > 0 || files_copied == 0) }
}

macro_rules! generate_session_dir_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generate_session_dir_path in module {}", module_path!());
    };
}

mkfn!{
    generate_session_dir_path_introspect!();
    # [doc = " Generates unique directory path of the form:"] # [doc = " {crate_dir}/s-{timestamp}-{random-number}-working"] fn generate_session_dir_path (crate_dir : & Path) -> PathBuf { let timestamp = timestamp_to_string (SystemTime :: now ()) ; debug ! ("generate_session_dir_path: timestamp = {}" , timestamp) ; let random_number = rng () . next_u32 () ; debug ! ("generate_session_dir_path: random_number = {}" , random_number) ; let (zeroes , timestamp) = timestamp . split_at (3) ; assert_eq ! (zeroes , "000") ; let directory_name = format ! ("s-{}-{}-working" , timestamp , random_number . to_base_fixed_len (CASE_INSENSITIVE)) ; debug ! ("generate_session_dir_path: directory_name = {}" , directory_name) ; let directory_path = crate_dir . join (directory_name) ; debug ! ("generate_session_dir_path: directory_path = {}" , directory_path . display ()) ; directory_path }
}

macro_rules! create_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_dir in module {}", module_path!());
    };
}

mkfn!{
    create_dir_introspect!();
    fn create_dir (sess : & Session , path : & Path , dir_tag : & str) { match std_fs :: create_dir_all (path) { Ok (()) => { debug ! ("{} directory created successfully" , dir_tag) ; } Err (err) => sess . dcx () . emit_fatal (errors :: CreateIncrCompDir { tag : dir_tag , path , err }) , } }
}

macro_rules! lock_directory_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lock_directory in module {}", module_path!());
    };
}

mkfn!{
    lock_directory_introspect!();
    # [doc = " Allocate the lock-file and lock it."] fn lock_directory (sess : & Session , session_dir : & Path) -> (flock :: Lock , PathBuf) { let lock_file_path = lock_file_path (session_dir) ; debug ! ("lock_directory() - lock_file: {}" , lock_file_path . display ()) ; match flock :: Lock :: new (& lock_file_path , false , true , true ,) { Ok (lock) => (lock , lock_file_path) , Err (lock_err) => { let is_unsupported_lock = flock :: Lock :: error_unsupported (& lock_err) ; sess . dcx () . emit_fatal (errors :: CreateLock { lock_err , session_dir , is_unsupported_lock , is_cargo : rustc_session :: utils :: was_invoked_from_cargo () , }) ; } } }
}

macro_rules! delete_session_dir_lock_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function delete_session_dir_lock_file in module {}", module_path!());
    };
}

mkfn!{
    delete_session_dir_lock_file_introspect!();
    fn delete_session_dir_lock_file (sess : & Session , lock_file_path : & Path) { if let Err (err) = safe_remove_file (lock_file_path) { sess . dcx () . emit_warn (errors :: DeleteLock { path : lock_file_path , err }) ; } }
}

macro_rules! find_source_directory_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_source_directory in module {}", module_path!());
    };
}

mkfn!{
    find_source_directory_introspect!();
    # [doc = " Finds the most recent published session directory that is not in the"] # [doc = " ignore-list."] fn find_source_directory (crate_dir : & Path , source_directories_already_tried : & FxHashSet < PathBuf > ,) -> Option < PathBuf > { let iter = crate_dir . read_dir () . unwrap () . filter_map (| e | e . ok () . map (| e | e . path ())) ; find_source_directory_in_iter (iter , source_directories_already_tried) }
}

macro_rules! find_source_directory_in_iter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_source_directory_in_iter in module {}", module_path!());
    };
}

mkfn!{
    find_source_directory_in_iter_introspect!();
    fn find_source_directory_in_iter < I > (iter : I , source_directories_already_tried : & FxHashSet < PathBuf > ,) -> Option < PathBuf > where I : Iterator < Item = PathBuf > , { let mut best_candidate = (UNIX_EPOCH , None) ; for session_dir in iter { debug ! ("find_source_directory_in_iter - inspecting `{}`" , session_dir . display ()) ; let Some (directory_name) = session_dir . file_name () . unwrap () . to_str () else { debug ! ("find_source_directory_in_iter - ignoring") ; continue ; } ; if source_directories_already_tried . contains (& session_dir) || ! is_session_directory (& directory_name) || ! is_finalized (& directory_name) { debug ! ("find_source_directory_in_iter - ignoring") ; continue ; } let timestamp = match extract_timestamp_from_session_dir (& directory_name) { Ok (timestamp) => timestamp , Err (e) => { debug ! ("unexpected incr-comp session dir: {}: {}" , session_dir . display () , e) ; continue ; } } ; if timestamp > best_candidate . 0 { best_candidate = (timestamp , Some (session_dir . clone ())) ; } } best_candidate . 1 }
}

macro_rules! is_finalized_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_finalized in module {}", module_path!());
    };
}

mkfn!{
    is_finalized_introspect!();
    fn is_finalized (directory_name : & str) -> bool { ! directory_name . ends_with ("-working") }
}

macro_rules! is_session_directory_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_session_directory in module {}", module_path!());
    };
}

mkfn!{
    is_session_directory_introspect!();
    fn is_session_directory (directory_name : & str) -> bool { directory_name . starts_with ("s-") && ! directory_name . ends_with (LOCK_FILE_EXT) }
}

macro_rules! is_session_directory_lock_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_session_directory_lock_file in module {}", module_path!());
    };
}

mkfn!{
    is_session_directory_lock_file_introspect!();
    fn is_session_directory_lock_file (file_name : & str) -> bool { file_name . starts_with ("s-") && file_name . ends_with (LOCK_FILE_EXT) }
}

macro_rules! extract_timestamp_from_session_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_timestamp_from_session_dir in module {}", module_path!());
    };
}

mkfn!{
    extract_timestamp_from_session_dir_introspect!();
    fn extract_timestamp_from_session_dir (directory_name : & str) -> Result < SystemTime , & 'static str > { if ! is_session_directory (directory_name) { return Err ("not a directory") ; } let dash_indices : Vec < _ > = directory_name . match_indices ('-') . map (| (idx , _) | idx) . collect () ; if dash_indices . len () != 3 { return Err ("not three dashes in name") ; } string_to_timestamp (& directory_name [dash_indices [0] + 1 .. dash_indices [1]]) }
}

macro_rules! timestamp_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function timestamp_to_string in module {}", module_path!());
    };
}

mkfn!{
    timestamp_to_string_introspect!();
    fn timestamp_to_string (timestamp : SystemTime) -> BaseNString { let duration = timestamp . duration_since (UNIX_EPOCH) . unwrap () ; let micros : u64 = duration . as_micros () . try_into () . unwrap () ; micros . to_base_fixed_len (CASE_INSENSITIVE) }
}

macro_rules! string_to_timestamp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function string_to_timestamp in module {}", module_path!());
    };
}

mkfn!{
    string_to_timestamp_introspect!();
    fn string_to_timestamp (s : & str) -> Result < SystemTime , & 'static str > { let micros_since_unix_epoch = match u64 :: from_str_radix (s , INT_ENCODE_BASE as u32) { Ok (micros) => micros , Err (_) => return Err ("timestamp not an int") , } ; let duration = Duration :: from_micros (micros_since_unix_epoch) ; Ok (UNIX_EPOCH + duration) }
}

macro_rules! crate_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crate_path in module {}", module_path!());
    };
}

mkfn!{
    crate_path_introspect!();
    fn crate_path (sess : & Session , crate_name : Symbol) -> PathBuf { let incr_dir = sess . opts . incremental . as_ref () . unwrap () . clone () ; let crate_types = collect_crate_types (sess , & []) ; let stable_crate_id = StableCrateId :: new (crate_name , crate_types . contains (& CrateType :: Executable) , sess . opts . cg . metadata . clone () , sess . cfg_version ,) ; let crate_name = format ! ("{crate_name}-{}" , stable_crate_id . as_u64 () . to_base_fixed_len (CASE_INSENSITIVE)) ; incr_dir . join (crate_name) }
}

macro_rules! is_old_enough_to_be_collected_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_old_enough_to_be_collected in module {}", module_path!());
    };
}

mkfn!{
    is_old_enough_to_be_collected_introspect!();
    fn is_old_enough_to_be_collected (timestamp : SystemTime) -> bool { timestamp < SystemTime :: now () - Duration :: from_secs (10) }
}

macro_rules! garbage_collect_session_directories_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function garbage_collect_session_directories in module {}", module_path!());
    };
}

mkfn!{
    garbage_collect_session_directories_introspect!();
    # [doc = " Runs garbage collection for the current session."] pub (crate) fn garbage_collect_session_directories (sess : & Session) -> io :: Result < () > { debug ! ("garbage_collect_session_directories() - begin") ; let session_directory = sess . incr_comp_session_dir () ; debug ! ("garbage_collect_session_directories() - session directory: {}" , session_directory . display ()) ; let crate_directory = session_directory . parent () . unwrap () ; debug ! ("garbage_collect_session_directories() - crate directory: {}" , crate_directory . display ()) ; let mut session_directories = FxIndexSet :: default () ; let mut lock_files = UnordSet :: default () ; for dir_entry in crate_directory . read_dir () ? { let Ok (dir_entry) = dir_entry else { continue ; } ; let entry_name = dir_entry . file_name () ; let Some (entry_name) = entry_name . to_str () else { continue ; } ; if is_session_directory_lock_file (& entry_name) { lock_files . insert (entry_name . to_string ()) ; } else if is_session_directory (& entry_name) { session_directories . insert (entry_name . to_string ()) ; } else { } } session_directories . sort () ; let lock_file_to_session_dir : UnordMap < String , Option < String > > = lock_files . into_items () . map (| lock_file_name | { assert ! (lock_file_name . ends_with (LOCK_FILE_EXT)) ; let dir_prefix_end = lock_file_name . len () - LOCK_FILE_EXT . len () ; let session_dir = { let dir_prefix = & lock_file_name [0 .. dir_prefix_end] ; session_directories . iter () . find (| dir_name | dir_name . starts_with (dir_prefix)) } ; (lock_file_name , session_dir . map (String :: clone)) }) . into () ; for (lock_file_name , directory_name) in lock_file_to_session_dir . items () . into_sorted_stable_ord () { if directory_name . is_none () { let Ok (timestamp) = extract_timestamp_from_session_dir (lock_file_name) else { debug ! ("found lock-file with malformed timestamp: {}" , crate_directory . join (& lock_file_name) . display ()) ; continue ; } ; let lock_file_path = crate_directory . join (& * lock_file_name) ; if is_old_enough_to_be_collected (timestamp) { debug ! ("garbage_collect_session_directories() - deleting \
                    garbage lock file: {}" , lock_file_path . display ()) ; delete_session_dir_lock_file (sess , & lock_file_path) ; } else { debug ! ("garbage_collect_session_directories() - lock file with \
                    no session dir not old enough to be collected: {}" , lock_file_path . display ()) ; } } } let lock_file_to_session_dir : UnordMap < String , String > = lock_file_to_session_dir . into_items () . filter_map (| (lock_file_name , directory_name) | directory_name . map (| n | (lock_file_name , n))) . into () ; for directory_name in session_directories { if ! lock_file_to_session_dir . items () . any (| (_ , dir) | * dir == directory_name) { let path = crate_directory . join (directory_name) ; if let Err (err) = std_fs :: remove_dir_all (& path) { sess . dcx () . emit_warn (errors :: InvalidGcFailed { path : & path , err }) ; } } } let deletion_candidates = lock_file_to_session_dir . items () . filter_map (| (lock_file_name , directory_name) | { debug ! ("garbage_collect_session_directories() - inspecting: {}" , directory_name) ; let Ok (timestamp) = extract_timestamp_from_session_dir (directory_name) else { debug ! ("found session-dir with malformed timestamp: {}" , crate_directory . join (directory_name) . display ()) ; return None ; } ; if is_finalized (directory_name) { let lock_file_path = crate_directory . join (lock_file_name) ; match flock :: Lock :: new (& lock_file_path , false , false , true ,) { Ok (lock) => { debug ! ("garbage_collect_session_directories() - \
                            successfully acquired lock") ; debug ! ("garbage_collect_session_directories() - adding \
                            deletion candidate: {}" , directory_name) ; return Some (((timestamp , crate_directory . join (directory_name)) , Some (lock) ,)) ; } Err (_) => { debug ! ("garbage_collect_session_directories() - \
                            not collecting, still in use") ; } } } else if is_old_enough_to_be_collected (timestamp) { let lock_file_path = crate_directory . join (lock_file_name) ; match flock :: Lock :: new (& lock_file_path , false , false , true ,) { Ok (lock) => { debug ! ("garbage_collect_session_directories() - \
                            successfully acquired lock") ; delete_old (sess , & crate_directory . join (directory_name)) ; drop (lock) ; } Err (_) => { debug ! ("garbage_collect_session_directories() - \
                            not collecting, still in use") ; } } } else { debug ! ("garbage_collect_session_directories() - not finalized, not \
                    old enough") ; } None }) ; let deletion_candidates = deletion_candidates . into () ; all_except_most_recent (deletion_candidates) . into_items () . all (| (path , lock) | { debug ! ("garbage_collect_session_directories() - deleting `{}`" , path . display ()) ; if let Err (err) = std_fs :: remove_dir_all (& path) { sess . dcx () . emit_warn (errors :: FinalizedGcFailed { path : & path , err }) ; } else { delete_session_dir_lock_file (sess , & lock_file_path (& path)) ; } drop (lock) ; true }) ; Ok (()) }
}

macro_rules! delete_old_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function delete_old in module {}", module_path!());
    };
}

mkfn!{
    delete_old_introspect!();
    fn delete_old (sess : & Session , path : & Path) { debug ! ("garbage_collect_session_directories() - deleting `{}`" , path . display ()) ; if let Err (err) = std_fs :: remove_dir_all (path) { sess . dcx () . emit_warn (errors :: SessionGcFailed { path , err }) ; } else { delete_session_dir_lock_file (sess , & lock_file_path (path)) ; } }
}

macro_rules! all_except_most_recent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function all_except_most_recent in module {}", module_path!());
    };
}

mkfn!{
    all_except_most_recent_introspect!();
    fn all_except_most_recent (deletion_candidates : UnordMap < (SystemTime , PathBuf) , Option < flock :: Lock > > ,) -> UnordMap < PathBuf , Option < flock :: Lock > > { let most_recent = deletion_candidates . items () . map (| (& (timestamp , _) , _) | timestamp) . max () ; if let Some (most_recent) = most_recent { deletion_candidates . into_items () . filter (| & ((timestamp , _) , _) | timestamp != most_recent) . map (| ((_ , path) , lock) | (path , lock)) . collect () } else { UnordMap :: default () } }
}

macro_rules! safe_remove_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function safe_remove_file in module {}", module_path!());
    };
}

mkfn!{
    safe_remove_file_introspect!();
    fn safe_remove_file (p : & Path) -> io :: Result < () > { match std_fs :: remove_file (p) { Err (err) if err . kind () == io :: ErrorKind :: NotFound => Ok (()) , result => result , } }
}

macro_rules! rename_path_with_retry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename_path_with_retry in module {}", module_path!());
    };
}

mkfn!{
    rename_path_with_retry_introspect!();
    fn rename_path_with_retry (from : & Path , to : & Path , mut retries_left : usize) -> std :: io :: Result < () > { loop { match std_fs :: rename (from , to) { Ok (()) => return Ok (()) , Err (e) => { if retries_left > 0 && e . kind () == ErrorKind :: PermissionDenied { std :: thread :: sleep (Duration :: from_millis (50)) ; retries_left -= 1 ; } else { return Err (e) ; } } } } }
}