// Generated macro for macro_2670 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2670 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2670"}
// Dependencies: {}
define_sql_function ! { # [doc = " Returns the IP address as text, ignoring the netmask."] # [cfg (feature = "postgres_backend")] fn host < T : InetOrCidr + SingleValue > (addr : T) -> Text ; }
};
}
