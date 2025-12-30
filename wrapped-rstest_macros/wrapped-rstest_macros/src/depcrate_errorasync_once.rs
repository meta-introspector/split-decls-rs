// Generated macro for async_once (function)
macro_rules! Depcrate_errorasync_once {
() => {
// Module: crate::error
// Provides: {"async_once"}
// Dependencies: {}
fn async_once < 'a > (test : & 'a ItemFn , info : & FixtureInfo) -> Errors < 'a > { match (test . sig . asyncness , info . arguments . get_once ()) { (Some (_asyncness) , Some (once)) => Box :: new (std :: iter :: once (syn :: Error :: new_spanned (once , "Cannot apply #[once] to async fixture." ,))) , _ => Box :: new (std :: iter :: empty ()) , } }
};
}
