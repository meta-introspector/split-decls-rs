// Generated macro for need_bitcode_in_object (function)
macro_rules! Depcrate_back_writeneed_bitcode_in_object {
() => {
// Module: crate::back::write
// Provides: {"need_bitcode_in_object"}
// Dependencies: {}
fn need_bitcode_in_object (tcx : TyCtxt < '_ >) -> bool { let sess = tcx . sess ; sess . opts . cg . embed_bitcode && tcx . crate_types () . contains (& CrateType :: Rlib) && sess . opts . output_types . contains_key (& OutputType :: Exe) }
};
}
