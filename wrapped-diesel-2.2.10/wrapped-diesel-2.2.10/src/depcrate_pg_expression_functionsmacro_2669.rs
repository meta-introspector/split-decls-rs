// Generated macro for macro_2669 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2669 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2669"}
// Dependencies: {}
define_sql_function ! { # [doc = " Returns the address's family: 4 for IPv4, 6 for IPv6."] # [cfg (feature = "postgres_backend")] fn family < T : InetOrCidr + SingleValue > (addr : T) -> Integer ; }
};
}
