// Generated macro for impl_1457 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_target_decorationsimpl_1457 {
() => {
// Module: crate::query_builder::upsert::on_conflict_target_decorations
// Provides: {"impl_1457"}
// Dependencies: {}
impl < T , P > DecoratableTarget < P > for T where P : Expression , P :: SqlType : BoolOrNullableBool , T : UndecoratedConflictTarget , { type FilterOutput = DecoratedConflictTarget < T , WhereClause < P > > ; fn filter_target (self , predicate : P) -> Self :: FilterOutput { DecoratedConflictTarget { target : self , where_clause : NoWhereClause . and (predicate) , } } }
};
}
