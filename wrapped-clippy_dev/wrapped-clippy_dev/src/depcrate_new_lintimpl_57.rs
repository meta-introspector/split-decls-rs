// Generated macro for impl_57 (impl)
macro_rules! Depcrate_new_lintimpl_57 {
() => {
// Module: crate::new_lint
// Provides: {"impl_57"}
// Dependencies: {}
impl < T > Context for io :: Result < T > { fn context < C : AsRef < str > > (self , text : C) -> Self { match self { Ok (t) => Ok (t) , Err (e) => { let message = format ! ("{}: {e}" , text . as_ref ()) ; Err (io :: Error :: other (message)) } , } } }
};
}
