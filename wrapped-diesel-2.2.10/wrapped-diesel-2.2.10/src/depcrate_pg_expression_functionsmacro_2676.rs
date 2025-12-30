// Generated macro for macro_2676 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2676 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2676"}
// Dependencies: {}
define_sql_function ! { # [doc = " Returns the network part of the address, zeroing out whatever is to the right of the"] # [doc = " netmask. (This is equivalent to casting the value to cidr.)"] # [cfg (feature = "postgres_backend")] fn network < T : InetOrCidr + SingleValue > (addr : T) -> Cidr ; }
};
}
