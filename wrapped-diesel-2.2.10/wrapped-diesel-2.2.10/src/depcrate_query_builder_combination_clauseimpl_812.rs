// Generated macro for impl_812 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_812 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_812"}
// Dependencies: {}
impl < Combinator , Rule , Source , Rhs , T > Insertable < T > for CombinationClause < Combinator , Rule , Source , Rhs > where T : Table , T :: AllColumns : NonAggregate , Self : Query , { type Values = InsertFromSelect < Self , T :: AllColumns > ; fn values (self) -> Self :: Values { InsertFromSelect :: new (self) } }
};
}
