// Generated macro for impl_42 (impl)
macro_rules! Depcrate_core_build_steps_checkimpl_42 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"impl_42"}
// Dependencies: {}
impl Step for PrepareStdRmetaSysroot { type Output = RmetaSysroot ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let stamp = builder . ensure (Std { build_compiler : self . build_compiler , target : self . target , crates : vec ! [] , }) ; let dir = builder . out . join (self . build_compiler . host) . join (format ! ("stage{}-std-rmeta-artifacts" , self . build_compiler . stage)) ; RmetaSysroot :: from_stamp (builder , stamp , self . target , & dir) } }
};
}
