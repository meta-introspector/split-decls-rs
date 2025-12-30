// Generated macro for deny_builtin_meta_unsafety (function)
macro_rules! Depcrate_validate_attrdeny_builtin_meta_unsafety {
() => {
// Module: crate::validate_attr
// Provides: {"deny_builtin_meta_unsafety"}
// Dependencies: {}
pub fn deny_builtin_meta_unsafety (diag : DiagCtxtHandle < '_ > , unsafety : Safety , name : & Path) { if let Safety :: Unsafe (unsafe_span) = unsafety { diag . emit_err (errors :: InvalidAttrUnsafe { span : unsafe_span , name : name . clone () }) ; } }
};
}
