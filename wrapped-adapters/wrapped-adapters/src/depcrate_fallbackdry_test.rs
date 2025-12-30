// Generated macro for dry_test (function)
macro_rules! Depcrate_fallbackdry_test {
() => {
// Module: crate::fallback
// Provides: {"dry_test"}
// Dependencies: {}
# [test] fn dry_test () { use icu_provider :: hello_world :: * ; struct TestProvider ; impl DataProvider < HelloWorldV1 > for TestProvider { fn load (& self , _ : DataRequest) -> Result < DataResponse < HelloWorldV1 > , DataError > { panic ! ("pretend this is super expensive") } } impl DryDataProvider < HelloWorldV1 > for TestProvider { fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { if req . id . locale . region . is_some () || req . id . locale . language . as_str () == "en" { Err (DataErrorKind :: IdentifierNotFound . into_error ()) } else { Ok (Default :: default ()) } } } let provider = LocaleFallbackProvider :: new (TestProvider , LocaleFallbacker :: new () . static_to_owned ()) ; assert_eq ! (provider . dry_load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& "de-CH" . parse () . unwrap ()) , .. Default :: default () }) . unwrap () . locale , "de" . parse ::< DataLocale > () . ok ()) ; assert_eq ! (provider . dry_load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& "en-GB" . parse () . unwrap ()) , .. Default :: default () }) . unwrap () . locale , Some (DataLocale :: default ())) ; }
};
}
