// Generated macro for impl_24 (impl)
macro_rules! Depcrate_typesimpl_24 {
() => {
// Module: crate::types
// Provides: {"impl_24"}
// Dependencies: {}
impl FromStr for Type { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "u8" => Type :: U8 , "u16" => Type :: U16 , "u32" => Type :: U32 , "u64" => Type :: U64 , "u128" => Type :: U128 , "usize" => Type :: Usize , "i8" => Type :: I8 , "i16" => Type :: I16 , "i32" => Type :: I32 , "i64" => Type :: I64 , "i128" => Type :: I128 , "isize" => Type :: Isize , "f32" => Type :: F32 , "f64" => Type :: F64 , "bool" => Type :: Bool , "str" => Type :: Str , "istr" => Type :: IStr , "__internal_Debug" => Type :: Debug , "__internal_Display" => Type :: Display , "__internal_FormatSequence" => Type :: FormatSequence , "[u8]" => Type :: U8Slice , "?" => Type :: Format , "[?]" => Type :: FormatSlice , "char" => Type :: Char , _ => return Err (()) , }) } }
};
}
