// Generated macro for download_component (function)
macro_rules! Depcrate_core_downloaddownload_component {
() => {
// Module: crate::core::download
// Provides: {"download_component"}
// Dependencies: {}
fn download_component < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , out : & Path , mode : DownloadSource , filename : String , prefix : & str , key : & str , destination : & str ,) { let dwn_ctx = dwn_ctx . as_ref () ; if dwn_ctx . exec_ctx . dry_run () { return ; } let cache_dst = dwn_ctx . bootstrap_cache_path . as_ref () . cloned () . unwrap_or_else (| | out . join ("cache")) ; let cache_dir = cache_dst . join (key) ; if ! cache_dir . exists () { t ! (fs :: create_dir_all (& cache_dir)) ; } let bin_root = out . join (dwn_ctx . host_target) . join (destination) ; let tarball = cache_dir . join (& filename) ; let (base_url , url , should_verify) = match mode { DownloadSource :: CI => { let dist_server = if dwn_ctx . llvm_assertions { dwn_ctx . stage0_metadata . config . artifacts_with_llvm_assertions_server . clone () } else { dwn_ctx . stage0_metadata . config . artifacts_server . clone () } ; let url = format ! ("{}/{filename}" , key . strip_suffix (& format ! ("-{}" , dwn_ctx . llvm_assertions)) . unwrap ()) ; (dist_server , url , false) } DownloadSource :: Dist => { let dist_server = env :: var ("RUSTUP_DIST_SERVER") . unwrap_or (dwn_ctx . stage0_metadata . config . dist_server . to_string ()) ; (dist_server , format ! ("dist/{key}/{filename}") , true) } } ; let checksum = if should_verify { let error = format ! ("src/stage0 doesn't contain a checksum for {url}. \
            Pre-built artifacts might not be available for this \
            target at this time, see https://doc.rust-lang.org/nightly\
            /rustc/platform-support.html for more information.") ; let sha256 = dwn_ctx . stage0_metadata . checksums_sha256 . get (& url) . expect (& error) ; if tarball . exists () { if verify (dwn_ctx . exec_ctx , & tarball , sha256) { unpack (dwn_ctx . exec_ctx , & tarball , & bin_root , prefix) ; return ; } else { dwn_ctx . exec_ctx . verbose (| | { println ! ("ignoring cached file {} due to failed verification" , tarball . display ()) }) ; remove (dwn_ctx . exec_ctx , & tarball) ; } } Some (sha256) } else if tarball . exists () { unpack (dwn_ctx . exec_ctx , & tarball , & bin_root , prefix) ; return ; } else { None } ; let mut help_on_error = "" ; if destination == "ci-rustc" { help_on_error = "ERROR: failed to download pre-built rustc from CI

NOTE: old builds get deleted after a certain time
HELP: if trying to compile an old commit of rustc, disable `download-rustc` in bootstrap.toml:

[rust]
download-rustc = false
" ; } download_file (dwn_ctx , out , & format ! ("{base_url}/{url}") , & tarball , help_on_error) ; if let Some (sha256) = checksum && ! verify (dwn_ctx . exec_ctx , & tarball , sha256) { panic ! ("failed to verify {}" , tarball . display ()) ; } unpack (dwn_ctx . exec_ctx , & tarball , & bin_root , prefix) ; }
};
}
