// Generated macro for contains_not_test (function)
macro_rules! Depcrate_cfg_not_testcontains_not_test {
() => {
// Module: crate::cfg_not_test
// Provides: {"contains_not_test"}
// Dependencies: {}
fn contains_not_test (list : Option < & [MetaItemInner] > , not : bool) -> bool { list . is_some_and (| list | { list . iter () . any (| item | { item . ident () . is_some_and (| ident | match ident . name { rustc_span :: sym :: not => contains_not_test (item . meta_item_list () , ! not) , rustc_span :: sym :: test => not , _ => contains_not_test (item . meta_item_list () , not) , }) }) }) }
};
}
