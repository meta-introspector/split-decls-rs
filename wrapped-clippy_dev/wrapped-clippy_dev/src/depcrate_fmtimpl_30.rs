// Generated macro for impl_30 (impl)
macro_rules! Depcrate_fmtimpl_30 {
() => {
// Module: crate::fmt
// Provides: {"impl_30"}
// Dependencies: {}
impl Error { fn display (& self) { match self { Self :: CheckFailed => { eprintln ! ("Formatting check failed!\nRun `cargo dev fmt` to update.") ; } , Self :: Io (err) => { eprintln ! ("error: {err}") ; } , Self :: Parse (path , line , msg) => { eprintln ! ("error parsing `{}:{line}`: {msg}" , path . display ()) ; } , } } }
};
}
