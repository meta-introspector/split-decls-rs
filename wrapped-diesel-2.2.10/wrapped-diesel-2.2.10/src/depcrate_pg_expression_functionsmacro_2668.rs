// Generated macro for macro_2668 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2668 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2668"}
// Dependencies: {}
define_sql_function ! { # [doc = " Computes the broadcast address for the address's network."] # [cfg (feature = "postgres_backend")] fn broadcast < T : InetOrCidr + SingleValue > (addr : T) -> Inet ; }
};
}
