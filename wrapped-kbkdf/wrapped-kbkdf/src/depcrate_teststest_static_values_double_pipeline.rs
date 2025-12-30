// Generated macro for test_static_values_double_pipeline (function)
macro_rules! Depcrate_teststest_static_values_double_pipeline {
() => {
// Module: crate::tests
// Provides: {"test_static_values_double_pipeline"}
// Dependencies: {}
# [test] fn test_static_values_double_pipeline () { type HmacSha256 = hmac :: Hmac < sha2 :: Sha256 > ; struct MockOutput ; impl KeySizeUser for MockOutput { type KeySize = U64 ; } for (v , i) in KNOWN_VALUES_DOUBLE_PIPELINE_HMAC_SHA256 . iter () . zip (0 ..) { let dbl_pipeline = DoublePipeline :: < HmacSha256 , MockOutput > :: default () ; assert_eq ! (dbl_pipeline . derive (Params :: builder (v . key) . use_l (v . use_l) . use_separator (v . use_separator) . use_counter (false) . with_label (v . label) . with_context (v . context) . build () ,) , Ok (Array ::< _ , _ >:: try_from (v . expected) . unwrap ()) , "key derivation failed for (index: {i}):\n{v:x?}") ; } }
};
}
