// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { mkbuildrs ! { check_cfg : "feature" , values = ["my_feature" , "another_feature"] ; cfg : "my_custom_flag" = "enabled" ; resource_req : { ram = "8GB" , cpu = "4" , instance_type = "c5.xlarge" } ; secret_req : ["DATABASE_URL" , "API_TOKEN"] ; } }
};
}
