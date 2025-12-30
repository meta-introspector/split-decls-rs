// Generated macro for CollationElement (struct)
macro_rules! Depcrate_elementsCollationElement {
() => {
// Module: crate::elements
// Provides: {"CollationElement"}
// Dependencies: {}
# [doc = " A collation element is a 64-bit value."] # [doc = ""] # [doc = " Bits 63..32 are the primary weight."] # [doc = " Bits 31..16 are the secondary weight."] # [doc = " Bits 15..14 are the case bits."] # [doc = " Bits 13..8 and 5..0 are the (bitwise discontiguous) tertiary weight."] # [doc = " Bits 7..6 the quaternary weight."] # [derive (Copy , Clone , Debug , PartialEq)] pub (crate) struct CollationElement (u64) ;
};
}
