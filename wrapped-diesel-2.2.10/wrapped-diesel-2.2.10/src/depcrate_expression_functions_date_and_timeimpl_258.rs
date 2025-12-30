// Generated macro for impl_258 (impl)
macro_rules! Depcrate_expression_functions_date_and_timeimpl_258 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"impl_258"}
// Dependencies: {}
impl < DB : Backend > QueryFragment < DB > for today { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("CURRENT_DATE") ; Ok (()) } }
};
}
