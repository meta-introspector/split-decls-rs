// Generated macro for derive_proptest_arbitrary (function)
macro_rules! Depcratederive_proptest_arbitrary {
() => {
// Module: crate
// Provides: {"derive_proptest_arbitrary"}
// Dependencies: {}
# [doc = " See module level documentation for more information."] # [proc_macro_derive (Arbitrary , attributes (proptest))] pub fn derive_proptest_arbitrary (input : pm :: TokenStream) -> pm :: TokenStream { derive :: impl_proptest_arbitrary (syn :: parse (input) . unwrap ()) . into () }
};
}
