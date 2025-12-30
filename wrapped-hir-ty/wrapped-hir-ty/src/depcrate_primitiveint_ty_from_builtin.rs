// Generated macro for int_ty_from_builtin (function)
macro_rules! Depcrate_primitiveint_ty_from_builtin {
() => {
// Module: crate::primitive
// Provides: {"int_ty_from_builtin"}
// Dependencies: {}
pub (super) fn int_ty_from_builtin (t : BuiltinInt) -> IntTy { match t { BuiltinInt :: Isize => IntTy :: Isize , BuiltinInt :: I8 => IntTy :: I8 , BuiltinInt :: I16 => IntTy :: I16 , BuiltinInt :: I32 => IntTy :: I32 , BuiltinInt :: I64 => IntTy :: I64 , BuiltinInt :: I128 => IntTy :: I128 , } }
};
}
