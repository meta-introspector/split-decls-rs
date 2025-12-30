// Generated macro for SpecialMatcher (enum)
macro_rules! Depcrate_transliterate_transliteratorSpecialMatcher {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"SpecialMatcher"}
// Dependencies: {}
enum SpecialMatcher < 'a > { Compound (& 'a str) , Quantifier (QuantifierKind , & 'a str) , Segment (Segment < 'a >) , UnicodeSet (CodePointInversionListAndStringList < 'a >) , AnchorStart , AnchorEnd , }
};
}
