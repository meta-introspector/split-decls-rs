// Generated macro for impl_1421 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_clauseimpl_1421 {
() => {
// Module: crate::query_builder::upsert::on_conflict_clause
// Provides: {"impl_1421"}
// Dependencies: {}
impl < Values , Target , Action , WhereClause > OnConflictValues < Values , Target , Action , WhereClause > { pub (crate) fn new (values : Values , target : Target , action : Action , where_clause : WhereClause ,) -> Self { OnConflictValues { values , target , action , where_clause , } } pub (crate) fn replace_where < Where , F > (self , f : F ,) -> OnConflictValues < Values , Target , Action , Where > where F : FnOnce (WhereClause) -> Where , { OnConflictValues :: new (self . values , self . target , self . action , f (self . where_clause)) } }
};
}
