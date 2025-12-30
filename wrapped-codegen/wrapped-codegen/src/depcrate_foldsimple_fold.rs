// Generated macro for simple_fold (function)
macro_rules! Depcrate_foldsimple_fold {
() => {
// Module: crate::fold
// Provides: {"simple_fold"}
// Dependencies: {}
fn simple_fold (item : & str , name : & TokenStream) -> TokenStream { let method = method_name (item) ; quote ! { f .# method (# name) } }
};
}
