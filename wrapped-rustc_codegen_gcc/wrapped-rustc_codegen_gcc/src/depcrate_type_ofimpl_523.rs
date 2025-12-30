// Generated macro for impl_523 (impl)
macro_rules! Depcrate_type_ofimpl_523 {
() => {
// Module: crate::type_of
// Provides: {"impl_523"}
// Dependencies: {}
impl < 'gcc , 'tcx > CodegenCx < 'gcc , 'tcx > { fn type_from_unsigned_integer (& self , i : Integer) -> Type < 'gcc > { use Integer :: * ; match i { I8 => self . type_u8 () , I16 => self . type_u16 () , I32 => self . type_u32 () , I64 => self . type_u64 () , I128 => self . type_u128 () , } } # [cfg (feature = "master")] pub fn type_int_from_ty (& self , t : ty :: IntTy) -> Type < 'gcc > { match t { ty :: IntTy :: Isize => self . type_isize () , ty :: IntTy :: I8 => self . type_i8 () , ty :: IntTy :: I16 => self . type_i16 () , ty :: IntTy :: I32 => self . type_i32 () , ty :: IntTy :: I64 => self . type_i64 () , ty :: IntTy :: I128 => self . type_i128 () , } } # [cfg (feature = "master")] pub fn type_uint_from_ty (& self , t : ty :: UintTy) -> Type < 'gcc > { match t { ty :: UintTy :: Usize => self . type_isize () , ty :: UintTy :: U8 => self . type_i8 () , ty :: UintTy :: U16 => self . type_i16 () , ty :: UintTy :: U32 => self . type_i32 () , ty :: UintTy :: U64 => self . type_i64 () , ty :: UintTy :: U128 => self . type_i128 () , } } }
};
}
