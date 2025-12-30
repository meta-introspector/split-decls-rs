// Generated macro for impl_1341 (impl)
macro_rules! Depcrate_query_builder_update_statement_changesetimpl_1341 {
() => {
// Module: crate::query_builder::update_statement::changeset
// Provides: {"impl_1341"}
// Dependencies: {}
impl < T , U , DB > QueryFragment < DB > for Assign < T , U > where DB : Backend , T : QueryFragment < DB > , U : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { QueryFragment :: walk_ast (& self . target , out . reborrow ()) ? ; out . push_sql (" = ") ; QueryFragment :: walk_ast (& self . expr , out . reborrow ()) } }
};
}
