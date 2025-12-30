// Generated macro for impl_2779 (impl)
macro_rules! Depcrate_pg_expression_operatorsimpl_2779 {
() => {
// Module: crate::pg::expression::operators
// Provides: {"impl_2779"}
// Dependencies: {}
impl < L , R > QueryFragment < Pg > for ArrayIndex < L , R > where L : QueryFragment < Pg > , R : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut out : crate :: query_builder :: AstPass < '_ , 'b , Pg > ,) -> crate :: result :: QueryResult < () > { self . array_expr . walk_ast (out . reborrow ()) ? ; out . push_sql ("[") ; self . index_expr . walk_ast (out . reborrow ()) ? ; out . push_sql ("]") ; Ok (()) } }
};
}
