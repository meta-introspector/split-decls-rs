// Generated macro for test_family_precedence (function)
macro_rules! Depcratetest_family_precedence {
() => {
// Module: crate
// Provides: {"test_family_precedence"}
// Dependencies: {}
# [doc = " Test that the last option with multiple conflicting families wins."] # [test] fn test_family_precedence () { let driver = ExportDriver :: new (["en" . parse () . unwrap () , "%en" . parse () . unwrap () , "@en" . parse () . unwrap () , "%zh-TW" . parse () . unwrap () , "^zh-TW" . parse () . unwrap () ,] , DeduplicationStrategy :: None . into () , LocaleFallbacker :: new_without_data () ,) ; assert_eq ! (driver . requested_families , [(icu :: locale :: langid ! ("en") . into () , DataLocaleFamilyAnnotations :: single ()) , (icu :: locale :: langid ! ("zh-TW") . into () , DataLocaleFamilyAnnotations :: without_descendants ()) ,] . into_iter () . collect ::< HashMap < _ , _ >> ()) ; }
};
}
