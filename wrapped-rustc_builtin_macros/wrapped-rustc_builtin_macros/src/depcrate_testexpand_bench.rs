// Generated macro for expand_bench (function)
macro_rules! Depcrate_testexpand_bench {
() => {
// Module: crate::test
// Provides: {"expand_bench"}
// Dependencies: {}
pub (crate) fn expand_bench (cx : & mut ExtCtxt < '_ > , attr_sp : Span , meta_item : & ast :: MetaItem , item : Annotatable ,) -> Vec < Annotatable > { check_builtin_macro_attribute (cx , meta_item , sym :: bench) ; warn_on_duplicate_attribute (cx , & item , sym :: bench) ; expand_test_or_bench (cx , attr_sp , item , true) }
};
}
