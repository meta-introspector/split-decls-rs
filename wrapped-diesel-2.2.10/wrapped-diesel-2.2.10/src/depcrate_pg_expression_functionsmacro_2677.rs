// Generated macro for macro_2677 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2677 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2677"}
// Dependencies: {}
define_sql_function ! { # [doc = " Sets the netmask length for an inet or cidr value."] # [doc = " For inet, the address part does not changes. For cidr, address bits to the right of the new"] # [doc = " netmask are set to zero."] # [cfg (feature = "postgres_backend")] fn set_masklen < T : InetOrCidr + SingleValue > (addr : T , len : Integer) -> T ; }
};
}
