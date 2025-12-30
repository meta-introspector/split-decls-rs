// Generated macro for macro_383 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_collationmacro_383 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::collation
// Provides: {"macro_383"}
// Dependencies: {}
enum_keyword ! (# [doc = " Collation parameter key for ordering by case."] # [doc = ""] # [doc = " If set to upper, causes upper case to sort before lower case. If set to lower, causes lower case to sort before upper case."] # [doc = " Useful for locales that have already supported ordering but require different order of cases. Affects case and tertiary levels."] # [doc = ""] # [doc = " The defails see [LDML](https://unicode.org/reports/tr35/tr35-collation.html#Case_Parameters)."] [Default] CollationCaseFirst { # [doc = " Upper case to be sorted before lower case"] ("upper" => Upper) , # [doc = " Lower case to be sorted before upper case"] ("lower" => Lower) , # [doc = " No special case ordering"] [default] ("false" => False) , } , "kf") ;
};
}
