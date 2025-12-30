// Generated macro for impl_1458 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_target_decorationsimpl_1458 {
() => {
// Module: crate::query_builder::upsert::on_conflict_target_decorations
// Provides: {"impl_1458"}
// Dependencies: {}
impl < T , U , P > DecoratableTarget < P > for DecoratedConflictTarget < T , U > where P : Expression , P :: SqlType : BoolOrNullableBool , U : WhereAnd < P > , { type FilterOutput = DecoratedConflictTarget < T , < U as WhereAnd < P > > :: Output > ; fn filter_target (self , predicate : P) -> Self :: FilterOutput { DecoratedConflictTarget { target : self . target , where_clause : self . where_clause . and (predicate) , } } }
};
}
