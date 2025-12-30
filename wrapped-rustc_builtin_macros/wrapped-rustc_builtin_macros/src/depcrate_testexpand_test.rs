// Generated macro for expand_test (function)
macro_rules! Depcrate_testexpand_test {
() => {
// Module: crate::test
// Provides: {"expand_test"}
// Dependencies: {}
pub (crate) fn expand_test (cx : & mut ExtCtxt < '_ > , attr_sp : Span , meta_item : & ast :: MetaItem , item : Annotatable ,) -> Vec < Annotatable > { check_builtin_macro_attribute (cx , meta_item , sym :: test) ; warn_on_duplicate_attribute (cx , & item , sym :: test) ; expand_test_or_bench (cx , attr_sp , item , false) }
};
}
