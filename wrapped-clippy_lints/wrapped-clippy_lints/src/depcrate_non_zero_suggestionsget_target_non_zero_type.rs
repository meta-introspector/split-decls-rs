// Generated macro for get_target_non_zero_type (function)
macro_rules! Depcrate_non_zero_suggestionsget_target_non_zero_type {
() => {
// Module: crate::non_zero_suggestions
// Provides: {"get_target_non_zero_type"}
// Dependencies: {}
fn get_target_non_zero_type (ty : Ty < '_ >) -> Option < & 'static str > { match ty . kind () { ty :: Uint (uint_ty) => Some (match uint_ty { ty :: UintTy :: U8 => "NonZeroU8" , ty :: UintTy :: U16 => "NonZeroU16" , ty :: UintTy :: U32 => "NonZeroU32" , ty :: UintTy :: U64 => "NonZeroU64" , ty :: UintTy :: U128 => "NonZeroU128" , ty :: UintTy :: Usize => "NonZeroUsize" , }) , ty :: Int (int_ty) => Some (match int_ty { ty :: IntTy :: I8 => "NonZeroI8" , ty :: IntTy :: I16 => "NonZeroI16" , ty :: IntTy :: I32 => "NonZeroI32" , ty :: IntTy :: I64 => "NonZeroI64" , ty :: IntTy :: I128 => "NonZeroI128" , ty :: IntTy :: Isize => "NonZeroIsize" , }) , _ => None , } }
};
}
