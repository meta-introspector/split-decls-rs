// Generated macro for uint_ty_from_builtin (function)
macro_rules! Depcrate_primitiveuint_ty_from_builtin {
() => {
// Module: crate::primitive
// Provides: {"uint_ty_from_builtin"}
// Dependencies: {}
pub (super) fn uint_ty_from_builtin (t : BuiltinUint) -> UintTy { match t { BuiltinUint :: Usize => UintTy :: Usize , BuiltinUint :: U8 => UintTy :: U8 , BuiltinUint :: U16 => UintTy :: U16 , BuiltinUint :: U32 => UintTy :: U32 , BuiltinUint :: U64 => UintTy :: U64 , BuiltinUint :: U128 => UintTy :: U128 , } }
};
}
