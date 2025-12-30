// Generated macro for impl_367 (impl)
macro_rules! Depcrate_expression_countimpl_367 {
() => {
// Module: crate::expression::count
// Provides: {"impl_367"}
// Dependencies: {}
impl < DB : Backend > QueryFragment < DB > for CountStar { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("COUNT(*)") ; Ok (()) } }
};
}
