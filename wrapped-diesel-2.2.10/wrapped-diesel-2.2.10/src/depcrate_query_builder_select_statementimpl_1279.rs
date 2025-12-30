// Generated macro for impl_1279 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1279 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1279"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC > SelectStatementAccessor for SelectStatement < F , S , D , W , O , LOf , G , H , LC > { type Select = S ; type From = F ; type Distinct = D ; type Where = W ; type Order = O ; type LimitOffset = LOf ; type GroupBy = G ; type Having = H ; type Locking = LC ; fn select_clause (& self) -> & Self :: Select { & self . select } fn from_clause (& self) -> & Self :: From { & self . from } fn distinct_clause (& self) -> & Self :: Distinct { & self . distinct } fn where_clause (& self) -> & Self :: Where { & self . where_clause } fn order_clause (& self) -> & Self :: Order { & self . order } fn limit_offset_clause (& self) -> & Self :: LimitOffset { & self . limit_offset } fn group_by_clause (& self) -> & Self :: GroupBy { & self . group_by } fn having_clause (& self) -> & Self :: Having { & self . having } fn locking_clause (& self) -> & Self :: Locking { & self . locking } }
};
}
