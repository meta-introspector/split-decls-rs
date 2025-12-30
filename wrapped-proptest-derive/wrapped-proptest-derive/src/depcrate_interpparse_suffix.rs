// Generated macro for parse_suffix (function)
macro_rules! Depcrate_interpparse_suffix {
() => {
// Module: crate::interp
// Provides: {"parse_suffix"}
// Dependencies: {}
# [doc = " Parse a suffix of an integer literal."] fn parse_suffix (lit : & str) -> Option < & 'static str > { ["i8" , "i16" , "i32" , "i64" , "i128" , "isize" , "u8" , "u16" , "u32" , "u64" , "u128" , "usize" ,] . iter () . find (| s | lit . ends_with (* s)) . copied () }
};
}
