// Generated macro for impl_2033 (impl)
macro_rules! Depcrate_upsert_on_conflict_extensionimpl_2033 {
() => {
// Module: crate::upsert::on_conflict_extension
// Provides: {"impl_2033"}
// Dependencies: {}
impl < T , U , Op , Ret , Target , Action , WhereClause , Predicate > FilterDsl < Predicate > for InsertStatement < T , OnConflictValues < U , Target , Action , WhereClause > , Op , Ret > where T : QuerySource , WhereClause : WhereAnd < Predicate > , { type Output = InsertStatement < T , OnConflictValues < U , Target , Action , WhereClause :: Output > , Op , Ret > ; fn filter (self , predicate : Predicate) -> Self :: Output { self . replace_values (| values | { values . replace_where (| where_clause | where_clause . and (predicate)) }) } }
};
}
