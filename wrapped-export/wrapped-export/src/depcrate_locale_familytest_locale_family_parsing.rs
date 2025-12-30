// Generated macro for test_locale_family_parsing (function)
macro_rules! Depcrate_locale_familytest_locale_family_parsing {
() => {
// Module: crate::locale_family
// Provides: {"test_locale_family_parsing"}
// Dependencies: {}
# [test] fn test_locale_family_parsing () { let valid_families = ["und" , "de-CH" , "^es" , "@pt-BR" , "%en-001" , "full"] ; let invalid_families = ["invalid" , "@invalid" , "-foo" , "@full" , "full-001"] ; for family_str in valid_families { let family = family_str . parse :: < DataLocaleFamily > () . unwrap () ; let family_to_str = family . to_string () ; assert_eq ! (family_str , family_to_str) ; } for family_str in invalid_families { assert ! (family_str . parse ::< DataLocaleFamily > () . is_err ()) ; } }
};
}
