// Generated macro for impl_1869 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1869 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1869"}
// Dependencies: {}
impl < Left , Right , Kind , DB > QueryFragment < DB > for Join < Left , Right , Kind > where DB : Backend + DieselReserveSpecialization , Left : QuerySource , Left :: FromClause : QueryFragment < DB > , Right : QuerySource , Right :: FromClause : QueryFragment < DB > , Kind : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . left . from_clause . walk_ast (out . reborrow ()) ? ; self . kind . walk_ast (out . reborrow ()) ? ; out . push_sql (" JOIN ") ; self . right . from_clause . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
