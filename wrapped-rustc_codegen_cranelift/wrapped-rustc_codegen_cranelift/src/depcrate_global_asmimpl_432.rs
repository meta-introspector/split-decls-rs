// Generated macro for impl_432 (impl)
macro_rules! Depcrate_global_asmimpl_432 {
() => {
// Module: crate::global_asm
// Provides: {"impl_432"}
// Dependencies: {}
impl GlobalAsmConfig { pub (crate) fn new (tcx : TyCtxt < '_ >) -> Self { GlobalAsmConfig { assembler : crate :: toolchain :: get_toolchain_binary (tcx . sess , "as") , target : match & tcx . sess . opts . target_triple { rustc_target :: spec :: TargetTuple :: TargetTuple (triple) => triple . clone () , rustc_target :: spec :: TargetTuple :: TargetJson { path_for_rustdoc , .. } => { path_for_rustdoc . to_str () . unwrap () . to_owned () } } , output_filenames : tcx . output_filenames (()) . clone () , } } }
};
}
