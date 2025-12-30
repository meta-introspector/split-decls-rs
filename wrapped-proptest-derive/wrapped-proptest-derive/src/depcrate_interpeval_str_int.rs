// Generated macro for eval_str_int (function)
macro_rules! Depcrate_interpeval_str_int {
() => {
// Module: crate::interp
// Provides: {"eval_str_int"}
// Dependencies: {}
# [doc = " Interprets an integer literal in a string."] fn eval_str_int (lit : & str) -> Option < u128 > { let val = parse_lit_int (lit) ? ; let checked_val = if let Some (suffix) = parse_suffix (lit) { match suffix { "i8" if val <= i8 :: MAX as u128 => val , "i16" if val <= i16 :: MAX as u128 => val , "i32" if val <= i32 :: MAX as u128 => val , "i64" if val <= i64 :: MAX as u128 => val , "u8" if val <= u128 :: from (u8 :: MAX) => val , "u16" if val <= u128 :: from (u16 :: MAX) => val , "u32" if val <= u128 :: from (u32 :: MAX) => val , "u64" if val <= u128 :: from (u64 :: MAX) => val , "usize" if val <= usize :: MAX as u128 => val , "isize" if val <= isize :: MAX as u128 => val , "u128" => val , "i128" if val <= i128 :: MAX as u128 => val , _ => return None , } } else { val } ; Some (checked_val) }
};
}
