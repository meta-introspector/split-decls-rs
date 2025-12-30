// Generated macro for impl_1874 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1874 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1874"}
// Dependencies: {}
impl < Join , On , Selection > AppendSelection < Selection > for JoinOn < Join , On > where Join : AppendSelection < Selection > , { type Output = Join :: Output ; fn append_selection (& self , selection : Selection) -> Self :: Output { self . join . append_selection (selection) } }
};
}
