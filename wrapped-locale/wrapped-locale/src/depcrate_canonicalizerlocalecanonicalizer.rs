// Generated macro for LocaleCanonicalizer (struct)
macro_rules! Depcrate_canonicalizerLocaleCanonicalizer {
() => {
// Module: crate::canonicalizer
// Provides: {"LocaleCanonicalizer"}
// Dependencies: {}
# [doc = " Implements the algorithm defined in *[UTS #35: Annex C, LocaleId Canonicalization]*."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::Locale;"] # [doc = " use icu::locale::{LocaleCanonicalizer, TransformResult};"] # [doc = ""] # [doc = " let lc = LocaleCanonicalizer::new_extended();"] # [doc = ""] # [doc = " let mut locale: Locale = \"ja-Latn-fonipa-hepburn-heploc\".parse().unwrap();"] # [doc = " assert_eq!(lc.canonicalize(&mut locale), TransformResult::Modified);"] # [doc = " assert_eq!(locale, \"ja-Latn-alalc97-fonipa\".parse().unwrap());"] # [doc = " ```"] # [doc = ""] # [doc = " [UTS #35: Annex C, LocaleId Canonicalization]: https://unicode.org/reports/tr35/#LocaleId_Canonicalization"] # [derive (Debug)] pub struct LocaleCanonicalizer < Expander = LocaleExpander > { # [doc = " Data to support canonicalization."] aliases : DataPayload < LocaleAliasesV1 > , # [doc = " Likely subtags implementation for delegation."] expander : Expander , }
};
}
