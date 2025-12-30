// Generated macro for ArgAbiExt (trait)
macro_rules! Depcrate_abi_pass_modeArgAbiExt {
() => {
// Module: crate::abi::pass_mode
// Provides: {"ArgAbiExt"}
// Dependencies: {}
pub (super) trait ArgAbiExt < 'tcx > { fn get_abi_param (& self , tcx : TyCtxt < 'tcx >) -> SmallVec < [AbiParam ; 2] > ; fn get_abi_return (& self , tcx : TyCtxt < 'tcx >) -> (Option < AbiParam > , Vec < AbiParam >) ; }
};
}
