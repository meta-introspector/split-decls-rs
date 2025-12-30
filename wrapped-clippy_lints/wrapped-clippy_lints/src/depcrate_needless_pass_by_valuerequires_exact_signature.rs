// Generated macro for requires_exact_signature (function)
macro_rules! Depcrate_needless_pass_by_valuerequires_exact_signature {
() => {
// Module: crate::needless_pass_by_value
// Provides: {"requires_exact_signature"}
// Dependencies: {}
# [doc = " Functions marked with these attributes must have the exact signature."] pub (crate) fn requires_exact_signature (attrs : & [Attribute]) -> bool { attrs . iter () . any (Attribute :: is_proc_macro_attr) }
};
}
