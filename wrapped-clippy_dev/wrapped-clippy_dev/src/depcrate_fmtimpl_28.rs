// Generated macro for impl_28 (impl)
macro_rules! Depcrate_fmtimpl_28 {
() => {
// Module: crate::fmt
// Provides: {"impl_28"}
// Dependencies: {}
impl Error { fn display (& self) { match self { Self :: CheckFailed => { eprintln ! ("Formatting check failed!\nRun `cargo dev fmt` to update.") ; } , Self :: Io (err) => { eprintln ! ("error: {err}") ; } , Self :: Parse (path , line , msg) => { eprintln ! ("error parsing `{}:{line}`: {msg}" , path . display ()) ; } , } } }
};
}
