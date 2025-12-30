// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_type_impl_1175 {
() => {
// Module: crate::type_
// Provides: {"impl_1175"}
// Dependencies: {}
impl < 'll , 'tcx > CodegenCx < 'll , 'tcx > { pub (crate) fn type_bool (& self) -> & 'll Type { self . type_i8 () } pub (crate) fn type_int_from_ty (& self , t : ty :: IntTy) -> & 'll Type { match t { ty :: IntTy :: Isize => self . type_isize () , ty :: IntTy :: I8 => self . type_i8 () , ty :: IntTy :: I16 => self . type_i16 () , ty :: IntTy :: I32 => self . type_i32 () , ty :: IntTy :: I64 => self . type_i64 () , ty :: IntTy :: I128 => self . type_i128 () , } } pub (crate) fn type_uint_from_ty (& self , t : ty :: UintTy) -> & 'll Type { match t { ty :: UintTy :: Usize => self . type_isize () , ty :: UintTy :: U8 => self . type_i8 () , ty :: UintTy :: U16 => self . type_i16 () , ty :: UintTy :: U32 => self . type_i32 () , ty :: UintTy :: U64 => self . type_i64 () , ty :: UintTy :: U128 => self . type_i128 () , } } pub (crate) fn type_float_from_ty (& self , t : ty :: FloatTy) -> & 'll Type { match t { ty :: FloatTy :: F16 => self . type_f16 () , ty :: FloatTy :: F32 => self . type_f32 () , ty :: FloatTy :: F64 => self . type_f64 () , ty :: FloatTy :: F128 => self . type_f128 () , } } # [doc = " Return an LLVM type that has at most the required alignment,"] # [doc = " and exactly the required size, as a best-effort padding array."] pub (crate) fn type_padding_filler (& self , size : Size , align : Align) -> & 'll Type { let unit = Integer :: approximate_align (self , align) ; let size = size . bytes () ; let unit_size = unit . size () . bytes () ; assert_eq ! (size % unit_size , 0) ; self . type_array (self . type_from_integer (unit) , size / unit_size) } }
};
}
