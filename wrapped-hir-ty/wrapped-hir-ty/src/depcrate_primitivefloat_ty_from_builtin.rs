// Generated macro for float_ty_from_builtin (function)
macro_rules! Depcrate_primitivefloat_ty_from_builtin {
() => {
// Module: crate::primitive
// Provides: {"float_ty_from_builtin"}
// Dependencies: {}
pub (super) fn float_ty_from_builtin (t : BuiltinFloat) -> FloatTy { match t { BuiltinFloat :: F16 => FloatTy :: F16 , BuiltinFloat :: F32 => FloatTy :: F32 , BuiltinFloat :: F64 => FloatTy :: F64 , BuiltinFloat :: F128 => FloatTy :: F128 , } }
};
}
