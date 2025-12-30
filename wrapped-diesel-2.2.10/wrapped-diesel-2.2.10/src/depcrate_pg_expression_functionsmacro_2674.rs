// Generated macro for macro_2674 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2674 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2674"}
// Dependencies: {}
define_sql_function ! { # [doc = " Returns the netmask length in bits."] # [cfg (feature = "postgres_backend")] fn masklen < T : InetOrCidr + SingleValue > (addr : T) -> Integer ; }
};
}
