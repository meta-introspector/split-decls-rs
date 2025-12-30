// Generated macro for macro_2667 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2667 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2667"}
// Dependencies: {}
define_sql_function ! { # [doc = " Creates an abbreviated display format as text."] # [cfg (feature = "postgres_backend")] fn abbrev < T : InetOrCidr + SingleValue > (addr : T) -> Text ; }
};
}
