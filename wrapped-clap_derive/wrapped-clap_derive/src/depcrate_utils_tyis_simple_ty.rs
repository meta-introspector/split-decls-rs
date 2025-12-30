// Generated macro for is_simple_ty (function)
macro_rules! Depcrate_utils_tyis_simple_ty {
() => {
// Module: crate::utils::ty
// Provides: {"is_simple_ty"}
// Dependencies: {}
pub (crate) fn is_simple_ty (ty : & Type , name : & str) -> bool { only_last_segment (ty) . map (| segment | { if let PathArguments :: None = segment . arguments { segment . ident == name } else { false } }) . unwrap_or (false) }
};
}
