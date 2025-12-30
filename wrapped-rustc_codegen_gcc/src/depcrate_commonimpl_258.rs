// Generated macro for impl_258 (impl)
macro_rules! Depcrate_commonimpl_258 {
() => {
// Module: crate::common
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'gcc , 'tcx > SignType < 'gcc , 'tcx > for Type < 'gcc > { fn is_signed (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_i8 (cx) || self . is_i16 (cx) || self . is_i32 (cx) || self . is_i64 (cx) || self . is_i128 (cx) } fn is_unsigned (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool { self . is_u8 (cx) || self . is_u16 (cx) || self . is_u32 (cx) || self . is_u64 (cx) || self . is_u128 (cx) } fn to_signed (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > { if self . is_u8 (cx) { cx . i8_type } else if self . is_u16 (cx) { cx . i16_type } else if self . is_u32 (cx) { cx . i32_type } else if self . is_u64 (cx) { cx . i64_type } else if self . is_u128 (cx) { cx . i128_type } else if self . is_uchar (cx) { cx . char_type } else if self . is_ushort (cx) { cx . short_type } else if self . is_uint (cx) { cx . int_type } else if self . is_ulong (cx) { cx . long_type } else if self . is_ulonglong (cx) { cx . longlong_type } else { * self } } fn to_unsigned (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > { if self . is_i8 (cx) { cx . u8_type } else if self . is_i16 (cx) { cx . u16_type } else if self . is_i32 (cx) { cx . u32_type } else if self . is_i64 (cx) { cx . u64_type } else if self . is_i128 (cx) { cx . u128_type } else if self . is_char (cx) { cx . uchar_type } else if self . is_short (cx) { cx . ushort_type } else if self . is_int (cx) { cx . uint_type } else if self . is_long (cx) { cx . ulong_type } else if self . is_longlong (cx) { cx . ulonglong_type } else { * self } } }
};
}
