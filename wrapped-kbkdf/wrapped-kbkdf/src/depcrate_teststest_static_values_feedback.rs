// Generated macro for test_static_values_feedback (function)
macro_rules! Depcrate_teststest_static_values_feedback {
() => {
// Module: crate::tests
// Provides: {"test_static_values_feedback"}
// Dependencies: {}
# [test] fn test_static_values_feedback () { type HmacSha256 = hmac :: Hmac < sha2 :: Sha256 > ; type HmacSha512 = hmac :: Hmac < sha2 :: Sha512 > ; for (v , i) in KNOWN_VALUES_FEEDBACK_HMAC_SHA256 . iter () . zip (0 ..) { let iv = v . iv . map (| iv | Array :: try_from (iv) . unwrap ()) ; let feedback = Feedback :: < HmacSha256 , HmacSha512 > :: new (iv . as_ref ()) ; assert_eq ! (feedback . derive (Params :: builder (v . key) . use_l (v . use_l) . use_separator (v . use_separator) . with_label (v . label) . with_context (v . context) . build ()) , Ok (Array ::< _ , _ >:: try_from (v . expected) . unwrap ()) , "key derivation failed for (index: {i}):\n{v:x?}") ; } }
};
}
