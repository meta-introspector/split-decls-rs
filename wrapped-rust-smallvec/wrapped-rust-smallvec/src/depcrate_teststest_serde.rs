// Generated macro for test_serde (function)
macro_rules! Depcrate_teststest_serde {
() => {
// Module: crate::tests
// Provides: {"test_serde"}
// Dependencies: {}
# [cfg (feature = "serde")] # [test] fn test_serde () { use bincode :: { config , deserialize } ; let mut small_vec : SmallVec < i32 , 2 > = SmallVec :: new () ; small_vec . push (1) ; let encoded = config () . limit (100) . serialize (& small_vec) . unwrap () ; let decoded : SmallVec < i32 , 2 > = deserialize (& encoded) . unwrap () ; assert_eq ! (small_vec , decoded) ; small_vec . push (2) ; small_vec . push (3) ; small_vec . push (4) ; let encoded = config () . limit (100) . serialize (& small_vec) . unwrap () ; let decoded : SmallVec < i32 , 2 > = deserialize (& encoded) . unwrap () ; assert_eq ! (small_vec , decoded) ; }
};
}
