// Generated macro for macro_96 (macro)
macro_rules! Depcrate_comparisonmacro_96 {
() => {
// Module: crate::comparison
// Provides: {"macro_96"}
// Dependencies: {}
icu_locale_core :: preferences :: define_preferences ! (# [doc = " The preferences for collation."] # [doc = ""] # [doc = " # Preferences"] # [doc = ""] # [doc = " Examples for using the different preferences below can be found in the [crate-level docs](crate)."] # [doc = ""] # [doc = " ## Case First"] # [doc = ""] # [doc = " See the [spec](https://www.unicode.org/reports/tr35/tr35-collation.html#Case_Parameters)."] # [doc = " This is the BCP47 key `kf`. Three possibilities: [`CollationCaseFirst::False`] (default,"] # [doc = " except for Danish and Maltese), [`CollationCaseFirst::Lower`], and [`CollationCaseFirst::Upper`]"] # [doc = " (default for Danish and Maltese)."] # [doc = ""] # [doc = " ## Numeric"] # [doc = ""] # [doc = " This is the BCP47 key `kn`. When set to [`CollationNumericOrdering::True`], any sequence of decimal"] # [doc = " digits (General_Category = Nd) is sorted at the primary level according to the"] # [doc = " numeric value. The default is [`CollationNumericOrdering::False`]."] [Copy] CollatorPreferences , { # [doc = " The collation type. This corresponds to the `-u-co` BCP-47 tag."] collation_type : CollationType , # [doc = " Treatment of case. (Large and small kana differences are treated as case differences.)"] # [doc = " This corresponds to the `-u-kf` BCP-47 tag."] case_first : CollationCaseFirst , # [doc = " When set to `True`, any sequence of decimal digits is sorted at a primary level according"] # [doc = " to the numeric value."] # [doc = " This corresponds to the `-u-kn` BPC-47 tag."] numeric_ordering : CollationNumericOrdering }) ;
};
}
