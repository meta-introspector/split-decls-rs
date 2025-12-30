// Generated macro for impl_2576 (impl)
macro_rules! Depcrate_pg_expression_arrayimpl_2576 {
() => {
// Module: crate::pg::expression::array
// Provides: {"impl_2576"}
// Dependencies: {}
impl < T , ST > QueryFragment < Pg > for ArrayLiteral < T , ST > where T : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> crate :: result :: QueryResult < () > { out . push_sql ("ARRAY[") ; QueryFragment :: walk_ast (& self . elements , out . reborrow ()) ? ; out . push_sql ("]") ; Ok (()) } }
};
}
