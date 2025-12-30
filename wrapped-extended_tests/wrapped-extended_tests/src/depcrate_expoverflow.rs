// Generated macro for overflow (function)
macro_rules! Depcrate_expoverflow {
() => {
// Module: crate::exp
// Provides: {"overflow"}
// Dependencies: {}
# [ignore = "experiment"] # [test] fn overflow () { let _guard = SERIALIZER . lock () . unwrap () ; let build_hasher = RandomState :: new () ; for b in [32 , 128 , 512 , 2048 , 8192 , 32768 , 131_072 , 524_288 , 2_097_152] { let r = 25 ; let mut buckets : Vec < usize > = vec ! [0 ; b] ; let n = (b / 32) * r * BUCKET_LEN ; for key in 0 .. n { # [allow (clippy :: cast_possible_truncation)] let hash = build_hasher . hash_one (key) as usize ; buckets [hash % b] += 1 ; } let mut o = 0 ; let mut m = 0 ; let mut map = Vec :: new () ; for c in & buckets { let offset = c . saturating_sub (BUCKET_LEN) ; if offset != 0 { o += 1 ; if m < offset { m = offset ; } } if map . len () < offset + 1 { map . resize (offset + 1 , 0) ; } map [offset] += 1 ; } println ! ("Num buckets: {b}, ratio: {r}/16, entries: {n}, overflows: {o}, max: {m}, rate: {:.4}%" , (to_f64 (o) / to_f64 (b)) * 100.0) ; for d in map . iter () . enumerate () { print ! ("{}:{} " , d . 0 , d . 1) ; } println ! () ; } }
};
}
