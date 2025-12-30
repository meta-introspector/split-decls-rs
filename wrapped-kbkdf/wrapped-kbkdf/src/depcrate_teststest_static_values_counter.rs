// Generated macro for test_static_values_counter (function)
macro_rules! Depcrate_teststest_static_values_counter {
() => {
// Module: crate::tests
// Provides: {"test_static_values_counter"}
// Dependencies: {}
# [test] fn test_static_values_counter () { type HmacSha256 = hmac :: Hmac < sha2 :: Sha256 > ; type HmacSha512 = hmac :: Hmac < sha2 :: Sha512 > ; let counter = Counter :: < HmacSha256 , HmacSha512 > :: default () ; for (v , i) in KNOWN_VALUES_COUNTER_HMAC_SHA256 . iter () . zip (0 ..) { assert_eq ! (counter . derive (Params :: builder (v . key) . use_l (v . use_l) . use_separator (v . use_separator) . with_label (v . label) . with_context (v . context) . build ()) , Ok (Array ::< _ , _ >:: try_from (v . expected) . unwrap ()) , "key derivation failed for (index: {i}):\n{v:x?}") ; } }
};
}
