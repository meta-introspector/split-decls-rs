// Generated macro for impl_375 (impl)
macro_rules! Depcrate_expression_countimpl_375 {
() => {
// Module: crate::expression::count
// Provides: {"impl_375"}
// Dependencies: {}
impl < T , E , DB > QueryFragment < DB > for CountDistinct < T , E > where T : SqlType + SingleValue , DB : Backend , E : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("COUNT(DISTINCT ") ; self . expr . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } }
};
}
