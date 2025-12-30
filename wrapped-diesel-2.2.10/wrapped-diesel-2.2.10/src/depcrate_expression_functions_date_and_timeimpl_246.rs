// Generated macro for impl_246 (impl)
macro_rules! Depcrate_expression_functions_date_and_timeimpl_246 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"impl_246"}
// Dependencies: {}
impl < DB : Backend > QueryFragment < DB > for now { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("CURRENT_TIMESTAMP") ; Ok (()) } }
};
}
