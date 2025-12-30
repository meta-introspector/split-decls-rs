// Generated macro for CollationElement32 (struct)
macro_rules! Depcrate_elementsCollationElement32 {
() => {
// Module: crate::elements
// Provides: {"CollationElement32"}
// Dependencies: {}
# [doc = " A compressed form of a collation element as stored in the collation"] # [doc = " data."] # [doc = ""] # [doc = " A `CollationElement32` can be \"normal\" or \"special\"."] # [doc = " Bits 7 and 6 are case bits for the \"normal\" case and setting"] # [doc = " both is an impossible case bit combination. Hence, \"special\""] # [doc = " `CollationElement32`s are marked by setting both case bits"] # [doc = " to 1. This is equivalent with the low byte being less than"] # [doc = " `SPECIAL_CE32_LOW_BYTE` (0xC0, i.e. 0b11000000) in the \"normal\""] # [doc = " case and equal to or greater in the \"special\" case."] # [doc = ""] # [doc = " For the normal case:"] # [doc = " Bits: 31..16: Primary weight"] # [doc = " Bits: 15..8: Secondary weight"] # [doc = " Bits:  7..6: Case bits (cannot both be 1 simultaneously)"] # [doc = " Bits:  5..0: The high part of the discontiguous tertiary weight"] # [doc = " (The quaternary weight and the low part of the discontiguous"] # [doc = " tertiary weight are zero.)"] # [doc = ""] # [doc = " For the special case:"] # [doc = " Bits 31..8: tag-specific; see the documentation for `Tag`."] # [doc = " Bits  7..6: The specialness marker; both bits set to 1"] # [doc = " Bits  5..4: Reserved. May be used in the future to indicate lccc!=0 and tccc!=0."] # [doc = " Bits  3..0: the tag (bit-compatible with `Tag`)"] # [derive (Copy , Clone , PartialEq , Debug)] pub (crate) struct CollationElement32 (u32) ;
};
}
