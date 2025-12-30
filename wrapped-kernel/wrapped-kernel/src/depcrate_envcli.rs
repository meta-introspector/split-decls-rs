// Generated macro for Cli (struct)
macro_rules! Depcrate_envCli {
() => {
// Module: crate::env
// Provides: {"Cli"}
// Dependencies: {}
# [derive (Debug)] struct Cli { # [allow (dead_code)] image_path : Option < String > , # [cfg (not (target_arch = "riscv64"))] freq : Option < u16 > , env_vars : HashMap < String , String , RandomState > , args : Vec < String > , # [allow (dead_code)] mmio : Vec < String > , }
};
}
