// Generated macro for impl_72 (impl)
macro_rules! Depcrate_builtin_typeimpl_72 {
() => {
// Module: crate::builtin_type
// Provides: {"impl_72"}
// Dependencies: {}
# [rustfmt :: skip] impl BuiltinFloat { pub fn from_suffix (suffix : & str) -> Option < BuiltinFloat > { let res = match suffix { "f16" => BuiltinFloat :: F16 , "f32" => BuiltinFloat :: F32 , "f64" => BuiltinFloat :: F64 , "f128" => BuiltinFloat :: F128 , _ => return None , } ; Some (res) } }
};
}
