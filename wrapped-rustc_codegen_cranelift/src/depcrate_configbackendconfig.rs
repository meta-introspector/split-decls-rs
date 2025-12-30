// Generated macro for BackendConfig (struct)
macro_rules! Depcrate_configBackendConfig {
() => {
// Module: crate::config
// Provides: {"BackendConfig"}
// Dependencies: {}
# [doc = " Configuration of cg_clif as passed in through `-Cllvm-args` and various env vars."] # [derive (Clone , Debug)] pub struct BackendConfig { # [doc = " Should the crate be AOT compiled or JIT executed."] # [doc = ""] # [doc = " Defaults to AOT compilation. Can be set using `-Cllvm-args=jit-mode`."] pub jit_mode : bool , # [doc = " When JIT mode is enable pass these arguments to the program."] # [doc = ""] # [doc = " Defaults to the value of `CG_CLIF_JIT_ARGS`."] pub jit_args : Vec < String > , }
};
}
