// Generated macro for impl_593 (impl)
macro_rules! Depcrate_expression_subselectimpl_593 {
() => {
// Module: crate::expression::subselect
// Provides: {"impl_593"}
// Dependencies: {}
impl < T , ST , DB > QueryFragment < DB > for Subselect < T , ST > where DB : Backend , T : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . values . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
