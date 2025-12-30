// Generated macro for clean_all (function)
macro_rules! Depcrate_cleanclean_all {
() => {
// Module: crate::clean
// Provides: {"clean_all"}
// Dependencies: {}
fn clean_all () -> Result < () , String > { let build_sysroot = get_sysroot_dir () ; let dirs_to_remove = ["target" . into () , build_sysroot . join ("sysroot") , build_sysroot . join ("sysroot_src") , build_sysroot . join ("target") ,] ; for dir in dirs_to_remove { let _ = remove_dir_all (dir) ; } let dirs_to_remove = ["regex" , "rand" , "simple-raytracer"] ; for dir in dirs_to_remove { let _ = remove_dir_all (Path :: new (crate :: BUILD_DIR) . join (dir)) ; } let files_to_remove = [build_sysroot . join ("Cargo.lock") , "perf.data" . into () , "perf.data.old" . into ()] ; for file in files_to_remove { let _ = remove_file (& file) ; } println ! ("Successfully ran `clean all`") ; Ok (()) }
};
}
