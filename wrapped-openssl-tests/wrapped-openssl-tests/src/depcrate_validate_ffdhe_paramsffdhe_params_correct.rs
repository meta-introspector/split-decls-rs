// Generated macro for ffdhe_params_correct (function)
macro_rules! Depcrate_validate_ffdhe_paramsffdhe_params_correct {
() => {
// Module: crate::validate_ffdhe_params
// Provides: {"ffdhe_params_correct"}
// Dependencies: {}
# [test] fn ffdhe_params_correct () { use NamedGroup :: * ; verify_openssl3_available () ; for (name , group) in [(FFDHE2048 , ffdhe_groups :: FFDHE2048) , (FFDHE3072 , ffdhe_groups :: FFDHE3072) , (FFDHE4096 , ffdhe_groups :: FFDHE4096) , (FFDHE6144 , ffdhe_groups :: FFDHE6144) , (FFDHE8192 , ffdhe_groups :: FFDHE8192) ,] { println ! ("testing {name:?}") ; test_ffdhe_params_correct (name , group) ; } }
};
}
