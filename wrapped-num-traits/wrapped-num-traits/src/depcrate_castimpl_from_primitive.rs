// Generated macro for impl_from_primitive (macro)
macro_rules! Depcrate_castimpl_from_primitive {
() => {
// Module: crate::cast
// Provides: {"impl_from_primitive"}
// Dependencies: {}
macro_rules ! impl_from_primitive { ($ T : ty , $ to_ty : ident) => { impl FromPrimitive for $ T { # [inline] fn from_isize (n : isize) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_i8 (n : i8) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_i16 (n : i16) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_i32 (n : i32) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_i64 (n : i64) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_i128 (n : i128) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_usize (n : usize) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_u8 (n : u8) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_u16 (n : u16) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_u32 (n : u32) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_u64 (n : u64) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_u128 (n : u128) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_f32 (n : f32) -> Option <$ T > { n .$ to_ty () } # [inline] fn from_f64 (n : f64) -> Option <$ T > { n .$ to_ty () } } } ; }
};
}
