// Generated macro for CanonicalCombiningClassMapBorrowed (struct)
macro_rules! Depcrate_propertiesCanonicalCombiningClassMapBorrowed {
() => {
// Module: crate::properties
// Provides: {"CanonicalCombiningClassMapBorrowed"}
// Dependencies: {}
# [doc = " Borrowed version of lookup of the Canonical_Combining_Class Unicode property."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::props::CanonicalCombiningClass;"] # [doc = " use icu::normalizer::properties::CanonicalCombiningClassMapBorrowed;"] # [doc = ""] # [doc = " let map = CanonicalCombiningClassMapBorrowed::new();"] # [doc = " assert_eq!(map.get('a'), CanonicalCombiningClass::NotReordered); // U+0061: LATIN SMALL LETTER A"] # [doc = " assert_eq!(map.get32(0x0301), CanonicalCombiningClass::Above); // U+0301: COMBINING ACUTE ACCENT"] # [doc = " ```"] # [derive (Debug)] pub struct CanonicalCombiningClassMapBorrowed < 'a > { # [doc = " The data trie"] decompositions : & 'a DecompositionData < 'a > , }
};
}
