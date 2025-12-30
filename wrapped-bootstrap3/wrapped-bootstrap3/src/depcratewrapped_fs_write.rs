// Generated macro for wrapped_fs_write (macro)
macro_rules! Depcratewrapped_fs_write {
() => {
// Module: crate
// Provides: {"wrapped_fs_write"}
// Dependencies: {}
macro_rules ! wrapped_fs_write { ($ path : expr , $ content : expr) => { { println ! ("🔧 WRAPPED: fs::write for: {}" , $ path . display ()) ; fs :: write ($ path , $ content) } } ; }
};
}
