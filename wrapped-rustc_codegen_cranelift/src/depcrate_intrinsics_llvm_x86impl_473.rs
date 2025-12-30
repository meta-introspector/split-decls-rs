// Generated macro for impl_473 (impl)
macro_rules! Depcrate_intrinsics_llvm_x86impl_473 {
() => {
// Module: crate::intrinsics::llvm_x86
// Provides: {"impl_473"}
// Dependencies: {}
impl PackSize { fn ret_clif_type (& self) -> Type { match self { Self :: U8 | Self :: S8 => types :: I8 , Self :: U16 | Self :: S16 => types :: I16 , } } fn src_clif_type (& self) -> Type { match self { Self :: U8 | Self :: S8 => types :: I16 , Self :: U16 | Self :: S16 => types :: I32 , } } fn src_ty < 'tcx > (& self , tcx : TyCtxt < 'tcx >) -> Ty < 'tcx > { match self { Self :: U8 | Self :: S8 => tcx . types . i16 , Self :: U16 | Self :: S16 => tcx . types . i32 , } } fn ret_ty < 'tcx > (& self , tcx : TyCtxt < 'tcx >) -> Ty < 'tcx > { match self { Self :: U8 => tcx . types . u8 , Self :: S8 => tcx . types . i8 , Self :: U16 => tcx . types . u16 , Self :: S16 => tcx . types . i16 , } } fn max (& self) -> i64 { match self { Self :: U8 => u8 :: MAX as u64 as i64 , Self :: S8 => i8 :: MAX as u8 as u64 as i64 , Self :: U16 => u16 :: MAX as u64 as i64 , Self :: S16 => i16 :: MAX as u64 as u64 as i64 , } } fn min (& self) -> i64 { match self { Self :: U8 | Self :: U16 => 0 , Self :: S8 => i16 :: from (i8 :: MIN) as u16 as i64 , Self :: S16 => i32 :: from (i16 :: MIN) as u32 as i64 , } } }
};
}
