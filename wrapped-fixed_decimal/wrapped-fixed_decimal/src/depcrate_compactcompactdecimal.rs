// Generated macro for CompactDecimal (struct)
macro_rules! Depcrate_compactCompactDecimal {
() => {
// Module: crate::compact
// Provides: {"CompactDecimal"}
// Dependencies: {}
# [doc = " A struct containing a [`Decimal`] significand together with an exponent, representing a"] # [doc = " number written in compact notation (such as 1.2M)."] # [doc = " This represents a _source number_, as defined"] # [doc = " [in UTS #35](https://www.unicode.org/reports/tr35/tr35-numbers.html#Plural_rules_syntax)."] # [doc = " The value exponent=0 represents a number in non-compact"] # [doc = " notation (such as 1\u{202f}200\u{202f}000)."] # [doc = ""] # [doc = " This is distinct from [`crate::ScientificDecimal`] because it does not represent leading 0s"] # [doc = " nor a sign in the exponent, and behaves differently in pluralization."] # [derive (Debug , Clone , PartialEq)] pub struct CompactDecimal { significand : Decimal , exponent : u8 , }
};
}
