// Generated macro for impl_1876 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1876 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1876"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for Inner where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" INNER") ; Ok (()) } }
};
}
