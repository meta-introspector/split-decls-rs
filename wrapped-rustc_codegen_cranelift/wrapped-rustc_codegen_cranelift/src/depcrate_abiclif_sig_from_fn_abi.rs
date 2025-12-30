// Generated macro for clif_sig_from_fn_abi (function)
macro_rules! Depcrate_abiclif_sig_from_fn_abi {
() => {
// Module: crate::abi
// Provides: {"clif_sig_from_fn_abi"}
// Dependencies: {}
fn clif_sig_from_fn_abi < 'tcx > (tcx : TyCtxt < 'tcx > , default_call_conv : CallConv , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > ,) -> Signature { let call_conv = conv_to_call_conv (tcx . sess , fn_abi . conv , default_call_conv) ; let inputs = fn_abi . args . iter () . flat_map (| arg_abi | arg_abi . get_abi_param (tcx) . into_iter ()) ; let (return_ptr , returns) = fn_abi . ret . get_abi_return (tcx) ; let params : Vec < _ > = return_ptr . into_iter () . chain (inputs) . collect () ; Signature { params , returns , call_conv } }
};
}
