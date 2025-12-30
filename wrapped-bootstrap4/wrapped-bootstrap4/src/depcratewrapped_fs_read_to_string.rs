// Generated macro for wrapped_fs_read_to_string (macro)
macro_rules! Depcratewrapped_fs_read_to_string {
() => {
// Module: crate
// Provides: {"wrapped_fs_read_to_string"}
// Dependencies: {}
macro_rules ! wrapped_fs_read_to_string { ($ path : expr) => { { println ! ("🔧 WRAPPED: fs::read_to_string for: {}" , $ path . display ()) ; fs :: read_to_string ($ path) } } ; }
};
}
