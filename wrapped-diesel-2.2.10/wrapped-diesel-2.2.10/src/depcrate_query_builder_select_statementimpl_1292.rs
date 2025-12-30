// Generated macro for impl_1292 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1292 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1292"}
// Dependencies: {}
impl < From , Selection > AppendSelection < Selection > for SelectStatement < From > where From : AsQuerySource , From :: QuerySource : AppendSelection < Selection > , { type Output = < From :: QuerySource as AppendSelection < Selection > > :: Output ; fn append_selection (& self , selection : Selection) -> Self :: Output { self . from . as_query_source () . append_selection (selection) } }
};
}
