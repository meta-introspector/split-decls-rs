// Generated macro for ParsedKey (enum)
macro_rules! DepcrateParsedKey {
() => {
// Module: crate
// Provides: {"ParsedKey"}
// Dependencies: {}
# [derive (Hash , PartialEq , Eq , Clone)] enum ParsedKey { Str (String) , Binary (Vec < u8 >) , Char (char) , I8 (i8) , I16 (i16) , I32 (i32) , I64 (i64) , I128 (i128) , Isize (isize) , U8 (u8) , U16 (u16) , U32 (u32) , U64 (u64) , U128 (u128) , Usize (usize) , Bool (bool) , Tuple (Vec < ParsedKey >) , # [cfg (feature = "unicase")] UniCase (UniCase < String >) , # [cfg (feature = "unicase")] UniCaseAscii (Ascii < String >) , # [cfg (feature = "uncased")] Uncased (Uncased < 'static >) , }
};
}
