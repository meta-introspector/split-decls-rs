// Generated macro for impl_1878 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1878 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1878"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for LeftOuter where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" LEFT OUTER") ; Ok (()) } }
};
}
