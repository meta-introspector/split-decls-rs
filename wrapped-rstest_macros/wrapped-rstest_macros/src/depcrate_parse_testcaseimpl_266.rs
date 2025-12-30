// Generated macro for impl_266 (impl)
macro_rules! Depcrate_parse_testcaseimpl_266 {
() => {
// Module: crate::parse::testcase
// Provides: {"impl_266"}
// Dependencies: {}
impl ToTokens for TestCase { fn to_tokens (& self , tokens : & mut TokenStream) { self . args . iter () . for_each (| c | c . to_tokens (tokens)) } }
};
}
