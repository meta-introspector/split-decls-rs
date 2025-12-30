// Generated macro for get_integer_from_float_constant (function)
macro_rules! Depcrate_floating_point_arithmeticget_integer_from_float_constant {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"get_integer_from_float_constant"}
// Dependencies: {}
# [expect (clippy :: cast_possible_truncation)] fn get_integer_from_float_constant (value : & Constant) -> Option < i32 > { match value { F32 (num) if num . fract () == 0.0 => { if (- 16_777_215.0 .. 16_777_216.0) . contains (num) { Some (num . round () as i32) } else { None } } , F64 (num) if num . fract () == 0.0 => { if (- 2_147_483_648.0 .. 2_147_483_648.0) . contains (num) { Some (num . round () as i32) } else { None } } , _ => None , } }
};
}
