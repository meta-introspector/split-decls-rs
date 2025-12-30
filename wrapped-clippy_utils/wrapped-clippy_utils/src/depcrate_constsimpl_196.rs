// Generated macro for impl_196 (impl)
macro_rules! Depcrate_constsimpl_196 {
() => {
// Module: crate::consts
// Provides: {"impl_196"}
// Dependencies: {}
impl IntTypeBounds for IntTy { type Output = i128 ; fn min_max (self) -> Option < (Self :: Output , Self :: Output) > { Some (match self { IntTy :: I8 => (i8 :: MIN . into () , i8 :: MAX . into ()) , IntTy :: I16 => (i16 :: MIN . into () , i16 :: MAX . into ()) , IntTy :: I32 => (i32 :: MIN . into () , i32 :: MAX . into ()) , IntTy :: I64 => (i64 :: MIN . into () , i64 :: MAX . into ()) , IntTy :: I128 => (i128 :: MIN , i128 :: MAX) , IntTy :: Isize => (isize :: MIN . try_into () . ok () ? , isize :: MAX . try_into () . ok () ?) , }) } fn bits (self) -> Self :: Output { match self { IntTy :: I8 => 8 , IntTy :: I16 => 16 , IntTy :: I32 => 32 , IntTy :: I64 => 64 , IntTy :: I128 => 128 , IntTy :: Isize => isize :: BITS . into () , } } }
};
}
