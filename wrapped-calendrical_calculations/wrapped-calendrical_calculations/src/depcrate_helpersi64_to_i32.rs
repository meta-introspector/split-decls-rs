// Generated macro for i64_to_i32 (function)
macro_rules! Depcrate_helpersi64_to_i32 {
() => {
// Module: crate::helpers
// Provides: {"i64_to_i32"}
// Dependencies: {}
# [doc = " Convert an i64 to i32 and with information on which way it was out of bounds if so"] # [inline] pub const fn i64_to_i32 (input : i64) -> Result < i32 , I32CastError > { if input < i32 :: MIN as i64 { Err (I32CastError :: BelowMin) } else if input > i32 :: MAX as i64 { Err (I32CastError :: AboveMax) } else { Ok (input as i32) } }
};
}
