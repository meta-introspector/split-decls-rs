// Generated macro for prepare_behaviour_dump_dir (function)
macro_rules! Depcrateprepare_behaviour_dump_dir {
() => {
// Module: crate
// Provides: {"prepare_behaviour_dump_dir"}
// Dependencies: {}
# [doc = " Ensures that the behavior dump directory is properly initialized."] pub fn prepare_behaviour_dump_dir (build : & Build) { static INITIALIZED : OnceLock < bool > = OnceLock :: new () ; let dump_path = build . out . join ("bootstrap-shims-dump") ; let initialized = INITIALIZED . get () . unwrap_or (& false) ; if ! initialized { if dump_path . exists () { t ! (fs :: remove_dir_all (& dump_path)) ; } t ! (fs :: create_dir_all (& dump_path)) ; t ! (INITIALIZED . set (true)) ; } }
};
}
