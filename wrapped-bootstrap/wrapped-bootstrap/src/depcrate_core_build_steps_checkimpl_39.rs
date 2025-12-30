// Generated macro for impl_39 (impl)
macro_rules! Depcrate_core_build_steps_checkimpl_39 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"impl_39"}
// Dependencies: {}
impl Step for PrepareRustcRmetaSysroot { type Output = RmetaSysroot ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let stamp = builder . ensure (Rustc :: from_build_compiler (self . build_compiler . clone () , self . target , vec ! [] ,)) ; let build_compiler = self . build_compiler . build_compiler () ; let dir = builder . out . join (build_compiler . host) . join (format ! ("stage{}-rustc-rmeta-artifacts" , build_compiler . stage + 1)) ; RmetaSysroot :: from_stamp (builder , stamp , self . target , & dir) } }
};
}
