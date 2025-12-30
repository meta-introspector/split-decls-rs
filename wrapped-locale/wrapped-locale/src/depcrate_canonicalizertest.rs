// Generated macro for test (module)
macro_rules! Depcrate_canonicalizertest {
() => {
// Module: crate::canonicalizer
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_uts35_rule_matches () { for (source , rule , result) in [("ja" , "und" , true) , ("und-heploc-hepburn" , "und-hepburn" , true) , ("ja-heploc-hepburn" , "und-hepburn" , true) , ("ja-hepburn" , "und-hepburn-heploc" , false) ,] { let source = source . parse () . unwrap () ; let rule = rule . parse :: < LanguageIdentifier > () . unwrap () ; assert_eq ! (uts35_rule_matches (& source , rule . language , rule . script , rule . region , rule . variants . iter () . map (Variant :: as_str) ,) , result , "{source}") ; } } # [test] fn test_uts35_replacement () { for (locale , rule_0 , rule_1 , result) in [("ja-Latn-fonipa-hepburn-heploc" , "und-hepburn-heploc" , "und-alalc97" , "ja-Latn-alalc97-fonipa" ,) , ("sgn-DD" , "und-DD" , "und-DE" , "sgn-DE") , ("sgn-DE" , "sgn-DE" , "gsg" , "gsg") ,] { let mut locale : Locale = locale . parse () . unwrap () ; let rule_0 = rule_0 . parse :: < LanguageIdentifier > () . unwrap () ; let rule_1 = rule_1 . parse () . unwrap () ; let result = result . parse :: < Locale > () . unwrap () ; uts35_replacement (& mut locale . id , ! rule_0 . language . is_unknown () , rule_0 . script . is_some () , rule_0 . region . is_some () , Some (rule_0 . variants . iter () . map (Variant :: as_str)) , & rule_1 ,) ; assert_eq ! (result , locale) ; } } }
};
}
