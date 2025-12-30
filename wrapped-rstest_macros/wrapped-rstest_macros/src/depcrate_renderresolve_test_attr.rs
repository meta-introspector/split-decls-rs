// Generated macro for resolve_test_attr (function)
macro_rules! Depcrate_renderresolve_test_attr {
() => {
// Module: crate::render
// Provides: {"resolve_test_attr"}
// Dependencies: {}
fn resolve_test_attr (test_attr : Option < & TestAttr > ,) -> Option < TokenStream > { match test_attr { Some (TestAttr :: Explicit (attr)) => { Some (quote ! { # attr }) } Some (TestAttr :: InAttrs) => { None } None => { Some (quote ! { # [test] }) } } }
};
}
