// Generated macro for get_test_spans (function)
macro_rules! Depcrate_doc_needless_doctest_mainget_test_spans {
() => {
// Module: crate::doc::needless_doctest_main
// Provides: {"get_test_spans"}
// Dependencies: {}
fn get_test_spans (item : & Item , ident : Ident , test_attr_spans : & mut Vec < Range < usize > >) { test_attr_spans . extend (item . attrs . iter () . find (| attr | attr . has_name (sym :: test)) . map (| attr | attr . span . lo () . to_usize () .. ident . span . hi () . to_usize ()) ,) ; }
};
}
