// Generated macro for impl_2797 (impl)
macro_rules! Depcrate_pg_expression_date_and_timeimpl_2797 {
() => {
// Module: crate::pg::expression::date_and_time
// Provides: {"impl_2797"}
// Dependencies: {}
impl < Ts , Tz > QueryFragment < Pg > for AtTimeZone < Ts , Tz > where Ts : QueryFragment < Pg > , Tz : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { self . timestamp . walk_ast (out . reborrow ()) ? ; out . push_sql (" AT TIME ZONE ") ; self . timezone . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
