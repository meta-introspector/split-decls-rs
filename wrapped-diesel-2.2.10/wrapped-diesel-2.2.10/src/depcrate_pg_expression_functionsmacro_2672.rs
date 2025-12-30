// Generated macro for macro_2672 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2672 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2672"}
// Dependencies: {}
define_sql_function ! { # [doc = " Computes the smallest network that includes both of the given networks."] # [cfg (feature = "postgres_backend")] fn inet_merge < T : InetOrCidr + SingleValue , U : InetOrCidr + SingleValue > (a : T , b : U) -> Cidr ; }
};
}
