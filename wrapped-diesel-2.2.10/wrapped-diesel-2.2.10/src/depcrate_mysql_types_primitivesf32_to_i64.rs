// Generated macro for f32_to_i64 (function)
macro_rules! Depcrate_mysql_types_primitivesf32_to_i64 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"f32_to_i64"}
// Dependencies: {}
# [allow (clippy :: cast_possible_truncation)] fn f32_to_i64 (f : f32) -> deserialize :: Result < i64 > { if f <= i64 :: MAX as f32 && f >= i64 :: MIN as f32 { Ok (f . trunc () as i64) } else { Err (Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _) } }
};
}
