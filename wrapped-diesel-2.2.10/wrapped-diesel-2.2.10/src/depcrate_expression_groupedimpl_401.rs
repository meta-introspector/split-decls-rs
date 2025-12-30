// Generated macro for impl_401 (impl)
macro_rules! Depcrate_expression_groupedimpl_401 {
() => {
// Module: crate::expression::grouped
// Provides: {"impl_401"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for Grouped < T > where T : QueryFragment < DB > , DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("(") ; self . 0 . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } }
};
}
