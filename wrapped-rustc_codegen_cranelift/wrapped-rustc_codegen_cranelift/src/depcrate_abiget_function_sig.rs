// Generated macro for get_function_sig (function)
macro_rules! Depcrate_abiget_function_sig {
() => {
// Module: crate::abi
// Provides: {"get_function_sig"}
// Dependencies: {}
pub (crate) fn get_function_sig < 'tcx > (tcx : TyCtxt < 'tcx > , default_call_conv : CallConv , inst : Instance < 'tcx > ,) -> Signature { assert ! (! inst . args . has_infer ()) ; clif_sig_from_fn_abi (tcx , default_call_conv , & FullyMonomorphizedLayoutCx (tcx) . fn_abi_of_instance (inst , ty :: List :: empty ()) ,) }
};
}
