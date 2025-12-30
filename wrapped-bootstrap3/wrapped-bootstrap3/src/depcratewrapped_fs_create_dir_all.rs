// Generated macro for wrapped_fs_create_dir_all (macro)
macro_rules! Depcratewrapped_fs_create_dir_all {
() => {
// Module: crate
// Provides: {"wrapped_fs_create_dir_all"}
// Dependencies: {}
macro_rules ! wrapped_fs_create_dir_all { ($ path : expr) => { { println ! ("🔧 WRAPPED: fs::create_dir_all for: {}" , $ path . display ()) ; fs :: create_dir_all ($ path) } } ; }
};
}
