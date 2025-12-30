// Generated macro for impl_1290 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1290 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1290"}
// Dependencies: {}
# [doc = " Allow `SelectStatement<From>` to act as if it were `From` as long as"] # [doc = " no other query methods have been called on it"] impl < From , T > AppearsInFromClause < T > for SelectStatement < From > where From : AsQuerySource , From :: QuerySource : AppearsInFromClause < T > + QuerySource , { type Count = < From :: QuerySource as AppearsInFromClause < T > > :: Count ; }
};
}
