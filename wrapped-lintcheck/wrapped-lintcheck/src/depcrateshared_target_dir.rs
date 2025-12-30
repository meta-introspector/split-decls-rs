// Generated macro for shared_target_dir (function)
macro_rules! Depcrateshared_target_dir {
() => {
// Module: crate
// Provides: {"shared_target_dir"}
// Dependencies: {}
# [doc = " The qualifier can be used to separate different threads from another. By"] # [doc = " default it should be set to `_<thread_id>`"] # [must_use] fn shared_target_dir (qualifier : & str) -> PathBuf { clippy_project_root () . join (format ! ("{}/lintcheck/shared_target_dir" , target_dir ())) . join (qualifier) }
};
}
