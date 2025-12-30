// Generated macro for prepare_std (function)
macro_rules! Depcrate_core_build_steps_checkprepare_std {
() => {
// Module: crate::core::build_steps::check
// Provides: {"prepare_std"}
// Dependencies: {}
# [doc = " Prepare the standard library for checking something (that requires stdlib) using"] # [doc = " `build_compiler`."] fn prepare_std (builder : & Builder < '_ > , build_compiler : Compiler , target : TargetSelection ,) -> Option < RmetaSysroot > { builder . std (build_compiler , builder . host_target) ; if builder . host_target != target { Some (builder . ensure (PrepareStdRmetaSysroot :: new (build_compiler , target))) } else { None } }
};
}
