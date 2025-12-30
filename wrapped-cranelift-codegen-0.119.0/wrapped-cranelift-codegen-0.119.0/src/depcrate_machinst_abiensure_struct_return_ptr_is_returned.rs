// Generated macro for ensure_struct_return_ptr_is_returned (function)
macro_rules! Depcrate_machinst_abiensure_struct_return_ptr_is_returned {
() => {
// Module: crate::machinst::abi
// Provides: {"ensure_struct_return_ptr_is_returned"}
// Dependencies: {}
fn ensure_struct_return_ptr_is_returned (sig : & ir :: Signature) -> ir :: Signature { let mut sig = sig . clone () ; if sig . uses_special_return (ArgumentPurpose :: StructReturn) { panic ! ("Explicit StructReturn return value not allowed: {sig:?}") } if let Some (struct_ret_index) = sig . special_param_index (ArgumentPurpose :: StructReturn) { if ! sig . returns . is_empty () { panic ! ("No return values are allowed when using StructReturn: {sig:?}") ; } sig . returns . insert (0 , sig . params [struct_ret_index]) ; } sig }
};
}
