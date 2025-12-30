// Generated macro for assert_eq_hash (function)
macro_rules! Depcrateassert_eq_hash {
() => {
// Module: crate
// Provides: {"assert_eq_hash"}
// Dependencies: {}
pub fn assert_eq_hash < T : std :: hash :: Hash > (v0 : T , v1 : T) { use std :: collections :: hash_map :: DefaultHasher ; use std :: hash :: Hasher ; let mut h0 = DefaultHasher :: default () ; v0 . hash (& mut h0) ; let mut h1 = DefaultHasher :: default () ; v1 . hash (& mut h1) ; assert_eq ! (h0 . finish () , h1 . finish ()) ; }
};
}
