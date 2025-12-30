// Generated macro for int_type_of_word (function)
macro_rules! Depcrate_attributes_reprint_type_of_word {
() => {
// Module: crate::attributes::repr
// Provides: {"int_type_of_word"}
// Dependencies: {}
fn int_type_of_word (s : Symbol) -> Option < IntType > { use IntType :: * ; match s { sym :: i8 => Some (SignedInt (IntTy :: I8)) , sym :: u8 => Some (UnsignedInt (UintTy :: U8)) , sym :: i16 => Some (SignedInt (IntTy :: I16)) , sym :: u16 => Some (UnsignedInt (UintTy :: U16)) , sym :: i32 => Some (SignedInt (IntTy :: I32)) , sym :: u32 => Some (UnsignedInt (UintTy :: U32)) , sym :: i64 => Some (SignedInt (IntTy :: I64)) , sym :: u64 => Some (UnsignedInt (UintTy :: U64)) , sym :: i128 => Some (SignedInt (IntTy :: I128)) , sym :: u128 => Some (UnsignedInt (UintTy :: U128)) , sym :: isize => Some (SignedInt (IntTy :: Isize)) , sym :: usize => Some (UnsignedInt (UintTy :: Usize)) , _ => None , } }
};
}
