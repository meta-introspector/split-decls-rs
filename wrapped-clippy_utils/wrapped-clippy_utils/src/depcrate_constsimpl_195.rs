// Generated macro for impl_195 (impl)
macro_rules! Depcrate_constsimpl_195 {
() => {
// Module: crate::consts
// Provides: {"impl_195"}
// Dependencies: {}
impl IntTypeBounds for UintTy { type Output = u128 ; fn min_max (self) -> Option < (Self :: Output , Self :: Output) > { Some (match self { UintTy :: U8 => (u8 :: MIN . into () , u8 :: MAX . into ()) , UintTy :: U16 => (u16 :: MIN . into () , u16 :: MAX . into ()) , UintTy :: U32 => (u32 :: MIN . into () , u32 :: MAX . into ()) , UintTy :: U64 => (u64 :: MIN . into () , u64 :: MAX . into ()) , UintTy :: U128 => (u128 :: MIN , u128 :: MAX) , UintTy :: Usize => (usize :: MIN . try_into () . ok () ? , usize :: MAX . try_into () . ok () ?) , }) } fn bits (self) -> Self :: Output { match self { UintTy :: U8 => 8 , UintTy :: U16 => 16 , UintTy :: U32 => 32 , UintTy :: U64 => 64 , UintTy :: U128 => 128 , UintTy :: Usize => usize :: BITS . into () , } } }
};
}
