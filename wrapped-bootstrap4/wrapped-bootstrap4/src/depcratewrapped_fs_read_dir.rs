// Generated macro for wrapped_fs_read_dir (macro)
macro_rules! Depcratewrapped_fs_read_dir {
() => {
// Module: crate
// Provides: {"wrapped_fs_read_dir"}
// Dependencies: {}
macro_rules ! wrapped_fs_read_dir { ($ path : expr) => { { println ! ("🔧 WRAPPED: fs::read_dir for: {}" , $ path . display ()) ; fs :: read_dir ($ path) } } ; }
};
}
