// Generated macro for tests (module)
macro_rules! Depcrate_provider_exceptionstests {
() => {
// Module: crate::provider::exceptions
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; fn test_roundtrip_once (exception : DecodedException) { let encoded = exception . encode () ; let encoded = zerovec :: ule :: encode_varule_to_box (& encoded) ; let decoded = encoded . decode () ; assert_eq ! (decoded , exception) ; } # [test] fn test_roundtrip () { test_roundtrip_once (DecodedException { lowercase : Some ('ø') , .. Default :: default () }) ; test_roundtrip_once (DecodedException { titlecase : Some ('X') , lowercase : Some ('ø') , .. Default :: default () }) ; test_roundtrip_once (DecodedException { titlecase : Some ('X') , .. Default :: default () }) ; test_roundtrip_once (DecodedException { titlecase : Some ('X') , simple_case_delta : Some (0xE999) , closure : Some ("hello world" . into ()) , .. Default :: default () }) ; test_roundtrip_once (DecodedException { simple_case_delta : Some (10) , closure : Some ("hello world" . into ()) , full : Some (["你好世界" . into () , "" . into () , "hi" . into () , "å" . into ()]) , .. Default :: default () }) ; test_roundtrip_once (DecodedException { closure : Some ("hello world" . into ()) , full : Some (["aa" . into () , "ț" . into () , "" . into () , "å" . into ()]) , .. Default :: default () }) ; test_roundtrip_once (DecodedException { full : Some (["你好世界" . into () , "" . into () , "hi" . into () , "å" . into ()]) , .. Default :: default () }) ; } }
};
}
