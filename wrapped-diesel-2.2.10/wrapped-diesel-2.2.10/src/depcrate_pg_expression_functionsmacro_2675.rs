// Generated macro for macro_2675 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2675 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2675"}
// Dependencies: {}
define_sql_function ! { # [doc = " Computes the network mask for the address's network."] # [cfg (feature = "postgres_backend")] fn netmask < T : InetOrCidr + SingleValue > (addr : T) -> Inet ; }
};
}
