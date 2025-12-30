// Generated macro for macro_2673 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2673 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2673"}
// Dependencies: {}
define_sql_function ! { # [doc = " Tests whether the addresses belong to the same IP family."] # [cfg (feature = "postgres_backend")] fn inet_same_family < T : InetOrCidr + SingleValue , U : InetOrCidr + SingleValue > (a : T , b : U) -> Bool ; }
};
}
