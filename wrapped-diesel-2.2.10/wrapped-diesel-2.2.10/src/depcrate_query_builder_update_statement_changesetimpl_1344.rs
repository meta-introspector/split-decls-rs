// Generated macro for impl_1344 (impl)
macro_rules! Depcrate_query_builder_update_statement_changesetimpl_1344 {
() => {
// Module: crate::query_builder::update_statement::changeset
// Provides: {"impl_1344"}
// Dependencies: {}
impl < DB , C > QueryFragment < DB > for ColumnWrapperForUpdate < C > where DB : Backend + DieselReserveSpecialization , C : Column , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_identifier (C :: NAME) } }
};
}
