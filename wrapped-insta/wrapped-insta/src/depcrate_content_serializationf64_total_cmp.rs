// Generated macro for f64_total_cmp (function)
macro_rules! Depcrate_content_serializationf64_total_cmp {
() => {
// Module: crate::content::serialization
// Provides: {"f64_total_cmp"}
// Dependencies: {}
fn f64_total_cmp (left : f64 , right : f64) -> Ordering { let mut left = left . to_bits () as i64 ; let mut right = right . to_bits () as i64 ; left ^= (((left >> 63) as u64) >> 1) as i64 ; right ^= (((right >> 63) as u64) >> 1) as i64 ; left . cmp (& right) }
};
}
