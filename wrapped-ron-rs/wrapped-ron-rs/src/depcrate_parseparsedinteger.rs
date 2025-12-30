// Generated macro for ParsedInteger (enum)
macro_rules! Depcrate_parseParsedInteger {
() => {
// Module: crate::parse
// Provides: {"ParsedInteger"}
// Dependencies: {}
pub enum ParsedInteger { I8 (i8) , I16 (i16) , I32 (i32) , I64 (i64) , # [cfg (feature = "integer128")] I128 (i128) , U8 (u8) , U16 (u16) , U32 (u32) , U64 (u64) , # [cfg (feature = "integer128")] U128 (u128) , }
};
}
