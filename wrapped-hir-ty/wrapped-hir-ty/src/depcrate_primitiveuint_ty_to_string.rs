// Generated macro for uint_ty_to_string (function)
macro_rules! Depcrate_primitiveuint_ty_to_string {
() => {
// Module: crate::primitive
// Provides: {"uint_ty_to_string"}
// Dependencies: {}
pub fn uint_ty_to_string (ty : UintTy) -> & 'static str { match ty { UintTy :: Usize => "usize" , UintTy :: U8 => "u8" , UintTy :: U16 => "u16" , UintTy :: U32 => "u32" , UintTy :: U64 => "u64" , UintTy :: U128 => "u128" , } }
};
}
