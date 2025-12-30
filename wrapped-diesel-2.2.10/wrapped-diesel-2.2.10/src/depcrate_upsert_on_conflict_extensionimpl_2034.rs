// Generated macro for impl_2034 (impl)
macro_rules! Depcrate_upsert_on_conflict_extensionimpl_2034 {
() => {
// Module: crate::upsert::on_conflict_extension
// Provides: {"impl_2034"}
// Dependencies: {}
impl < T , U , Op , Ret , Target , Action , WhereClause , Predicate > OrFilterDsl < Predicate > for InsertStatement < T , OnConflictValues < U , Target , Action , WhereClause > , Op , Ret > where T : QuerySource , WhereClause : WhereOr < Predicate > , { type Output = InsertStatement < T , OnConflictValues < U , Target , Action , WhereClause :: Output > , Op , Ret > ; fn or_filter (self , predicate : Predicate) -> Self :: Output { self . replace_values (| values | { values . replace_where (| where_clause | where_clause . or (predicate)) }) } }
};
}
