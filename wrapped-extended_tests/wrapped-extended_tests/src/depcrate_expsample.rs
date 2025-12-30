// Generated macro for sample (function)
macro_rules! Depcrate_expsample {
() => {
// Module: crate::exp
// Provides: {"sample"}
// Dependencies: {}
# [ignore = "experiment"] # [test] fn sample () { let _guard = SERIALIZER . lock () . unwrap () ; let build_hasher = RandomState :: new () ; for b in [2 , 8 , 32 , 128 , 512 , 2048 , 8192 , 32768 , 131_072 , 524_288 , 2_097_152 ,] { let mut buckets = vec ! [0 ; b] ; for key in 0 .. b * BUCKET_LEN { # [allow (clippy :: cast_possible_truncation)] let hash = build_hasher . hash_one (key) as usize ; buckets [hash % b] += 1 ; } println ! ("---------------") ; let mut s = 1 ; while s < b { let estimation : usize = buckets . iter () . take (s) . sum :: < usize > () * (b / s) ; println ! ("Num buckets: {b}, sample size: {s}, entries: {}, estimation: {estimation}, accuracy: {:.4}%" , b * BUCKET_LEN , (to_f64 (estimation) / to_f64 (b * BUCKET_LEN)) * 100.0) ; s *= 2 ; } println ! ("---------------") ; let c = b . trailing_zeros () . next_power_of_two () as usize ; let estimation : usize = buckets . iter () . take (c) . sum :: < usize > () * (b / c) ; println ! ("(Log2({b})) Num buckets: {b}, sample size: {c}, entries: {}, estimation: {estimation}, accuracy: {:.4}%" , b * BUCKET_LEN , (to_f64 (estimation) / to_f64 (b * BUCKET_LEN)) * 100.0) ; let estimation : usize = buckets . iter () . take (c * 2) . sum :: < usize > () * (b / (c * 2)) ; println ! ("(Log2({b}) * 2) Num buckets: {b}, sample size: {}, entries: {}, estimation: {estimation}, accuracy: {:.4}%" , c * 2 , b * BUCKET_LEN , (to_f64 (estimation) / to_f64 (b * BUCKET_LEN)) * 100.0) ; } }
};
}
