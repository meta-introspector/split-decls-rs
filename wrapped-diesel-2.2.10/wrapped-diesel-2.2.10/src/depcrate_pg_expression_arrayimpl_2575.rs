// Generated macro for impl_2575 (impl)
macro_rules! Depcrate_pg_expression_arrayimpl_2575 {
() => {
// Module: crate::pg::expression::array
// Provides: {"impl_2575"}
// Dependencies: {}
impl < T , ST > Expression for ArrayLiteral < T , ST > where ST : 'static , T : Expression , { type SqlType = sql_types :: Array < ST > ; }
};
}
