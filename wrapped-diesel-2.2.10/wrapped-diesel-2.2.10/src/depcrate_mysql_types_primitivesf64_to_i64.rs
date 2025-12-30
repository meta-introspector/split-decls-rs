// Generated macro for f64_to_i64 (function)
macro_rules! Depcrate_mysql_types_primitivesf64_to_i64 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"f64_to_i64"}
// Dependencies: {}
# [allow (clippy :: cast_possible_truncation)] fn f64_to_i64 (f : f64) -> deserialize :: Result < i64 > { if f <= i64 :: MAX as f64 && f >= i64 :: MIN as f64 { Ok (f . trunc () as i64) } else { Err (Box :: new (DeserializationError ("Numeric overflow/underflow occurred" . into () ,)) as _) } }
};
}
