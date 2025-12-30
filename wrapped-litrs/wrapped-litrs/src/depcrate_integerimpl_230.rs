// Generated macro for impl_230 (impl)
macro_rules! Depcrate_integerimpl_230 {
() => {
// Module: crate::integer
// Provides: {"impl_230"}
// Dependencies: {}
impl IntegerType { # [doc = " Returns the type corresponding to the given suffix (e.g. `\"u8\"` is"] # [doc = " mapped to `Self::U8`). If the suffix is not a valid integer type,"] # [doc = " `None` is returned."] pub fn from_suffix (suffix : & str) -> Option < Self > { match suffix { "u8" => Some (Self :: U8) , "u16" => Some (Self :: U16) , "u32" => Some (Self :: U32) , "u64" => Some (Self :: U64) , "u128" => Some (Self :: U128) , "usize" => Some (Self :: Usize) , "i8" => Some (Self :: I8) , "i16" => Some (Self :: I16) , "i32" => Some (Self :: I32) , "i64" => Some (Self :: I64) , "i128" => Some (Self :: I128) , "isize" => Some (Self :: Isize) , _ => None , } } # [doc = " Returns the suffix for this type, e.g. `\"u8\"` for `Self::U8`."] pub fn suffix (self) -> & 'static str { match self { Self :: U8 => "u8" , Self :: U16 => "u16" , Self :: U32 => "u32" , Self :: U64 => "u64" , Self :: U128 => "u128" , Self :: Usize => "usize" , Self :: I8 => "i8" , Self :: I16 => "i16" , Self :: I32 => "i32" , Self :: I64 => "i64" , Self :: I128 => "i128" , Self :: Isize => "isize" , } } }
};
}
