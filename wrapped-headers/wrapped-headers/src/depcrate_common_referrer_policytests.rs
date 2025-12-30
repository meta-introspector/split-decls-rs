// Generated macro for tests (module)
macro_rules! Depcrate_common_referrer_policytests {
() => {
// Module: crate::common::referrer_policy
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: test_decode ; use super :: ReferrerPolicy ; # [test] fn decode_as_last_policy () { assert_eq ! (test_decode ::< ReferrerPolicy > (& ["same-origin, origin"]) , Some (ReferrerPolicy :: ORIGIN) ,) ; assert_eq ! (test_decode ::< ReferrerPolicy > (& ["origin" , "same-origin"]) , Some (ReferrerPolicy :: SAME_ORIGIN) ,) ; } # [test] fn decode_as_last_known () { assert_eq ! (test_decode ::< ReferrerPolicy > (& ["origin, nope, nope, nope"]) , Some (ReferrerPolicy :: ORIGIN) ,) ; assert_eq ! (test_decode ::< ReferrerPolicy > (& ["nope, origin, nope, nope"]) , Some (ReferrerPolicy :: ORIGIN) ,) ; assert_eq ! (test_decode ::< ReferrerPolicy > (& ["nope, origin" , "nope, nope"]) , Some (ReferrerPolicy :: ORIGIN) ,) ; assert_eq ! (test_decode ::< ReferrerPolicy > (& ["nope" , "origin" , "nope, nope"]) , Some (ReferrerPolicy :: ORIGIN) ,) ; } # [test] fn decode_unknown () { assert_eq ! (test_decode ::< ReferrerPolicy > (& ["nope"]) , None ,) ; } # [test] fn matching () { let rp = ReferrerPolicy :: ORIGIN ; match rp { ReferrerPolicy :: ORIGIN => () , _ => panic ! ("matched wrong") , } } }
};
}
