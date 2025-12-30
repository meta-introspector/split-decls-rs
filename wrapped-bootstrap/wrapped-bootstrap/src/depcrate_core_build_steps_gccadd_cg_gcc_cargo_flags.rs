// Generated macro for add_cg_gcc_cargo_flags (function)
macro_rules! Depcrate_core_build_steps_gccadd_cg_gcc_cargo_flags {
() => {
// Module: crate::core::build_steps::gcc
// Provides: {"add_cg_gcc_cargo_flags"}
// Dependencies: {}
# [doc = " Configures a Cargo invocation so that it can build the GCC codegen backend."] pub fn add_cg_gcc_cargo_flags (cargo : & mut Cargo , gcc : & GccOutput) { cargo . rustflag (& format ! ("-L{}" , gcc . libgccjit . parent () . unwrap () . to_str () . unwrap ())) ; }
};
}
