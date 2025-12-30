// Generated macro for impl_611 (impl)
macro_rules! Depcrate_provider_pattern_item_uleimpl_611 {
() => {
// Module: crate::provider::pattern::item::ule
// Provides: {"impl_611"}
// Dependencies: {}
impl GenericPatternItemULE { # [doc = " Given the first byte of the three-byte array that `GenericPatternItemULE` encodes,"] # [doc = " the method determines whether the discriminant in"] # [doc = " the byte indicates that the array encodes the `GenericPatternItem::Field`"] # [doc = " or `GenericPatternItem::Literal` variant of the `GenericPatternItem`."] # [doc = ""] # [doc = " Returns true when it is a `GenericPatternItem::Field`."] # [inline] fn determine_field_from_u8 (byte : u8) -> bool { byte & 0b1000_0000 != 0 } # [inline] fn bytes_in_range (value : (& u8 , & u8 , & u8)) -> bool { if Self :: determine_field_from_u8 (* value . 0) { * value . 0 == 0b1000_0000 && * value . 1 == 0 && * value . 2 < 10 } else { let u = u32 :: from_be_bytes ([0x00 , * value . 0 , * value . 1 , * value . 2]) ; char :: try_from (u) . is_ok () } } # [doc = " Converts this [`GenericPatternItemULE`] to a [`PatternItemULE`]"] # [doc = " (if a Literal) or returns the placeholder value."] # [inline] pub (crate) fn as_pattern_item_ule (& self) -> Result < & PatternItemULE , u8 > { if Self :: determine_field_from_u8 (self . 0 [0]) { Err (self . 0 [2]) } else { if cfg ! (debug_assertions) { let GenericPatternItem :: Literal (c) = GenericPatternItem :: from_unaligned (* self) else { unreachable ! ("expected a literal!") } ; let pattern_item_ule = PatternItem :: Literal (c) . to_unaligned () ; debug_assert_eq ! (self . 0 , pattern_item_ule . 0) ; } Ok (unsafe { core :: mem :: transmute :: < & GenericPatternItemULE , & PatternItemULE > (self) }) } } }
};
}
