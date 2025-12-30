// Generated macro for is_signed_error (function)
macro_rules! Depcrate_types_cpp_constis_signed_error {
() => {
// Module: crate::types::cpp_const
// Provides: {"is_signed_error"}
// Dependencies: {}
fn is_signed_error (ty : & Type) -> bool { match ty { Type :: HRESULT => true , Type :: CppStruct (ty) => ! ty . def . underlying_type () . is_unsigned () , _ => false , } }
};
}
