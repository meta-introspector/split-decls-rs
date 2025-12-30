// Generated macro for impl_1286 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1286 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1286"}
// Dependencies: {}
impl < S , F , D , W , O , LOf , G , H , LC , QS > ValidSubselect < QS > for SelectStatement < FromClause < F > , S , D , W , O , LOf , G , H , LC > where Self : SelectQuery , F : QuerySource , QS : QuerySource , Join < F , QS , Inner > : QuerySource , W : ValidWhereClause < FromClause < Join < F , QS , Inner > > > , { }
};
}
