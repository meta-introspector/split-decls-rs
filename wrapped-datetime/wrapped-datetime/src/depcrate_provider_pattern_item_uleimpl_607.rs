// Generated macro for impl_607 (impl)
macro_rules! Depcrate_provider_pattern_item_uleimpl_607 {
() => {
// Module: crate::provider::pattern::item::ule
// Provides: {"impl_607"}
// Dependencies: {}
impl PatternItemULE { # [doc = " Given the first byte of the three-byte array that `PatternItemULE` encodes,"] # [doc = " the method determines whether the discriminant in"] # [doc = " the byte indicates that the array encodes the `PatternItem::Field`"] # [doc = " or `PatternItem::Literal` variant of the `PatternItem`."] # [doc = ""] # [doc = " Returns true when it is a `PatternItem::Field`."] # [inline] fn determine_field_from_u8 (byte : u8) -> bool { byte & 0b1000_0000 != 0 } # [inline] fn bytes_in_range (value : (& u8 , & u8 , & u8)) -> bool { if Self :: determine_field_from_u8 (* value . 0) { fields :: FieldULE :: validate_byte_pair ((* value . 1 , * value . 2)) . is_ok () && * value . 0 == 0b1000_0000 } else { char :: try_from (u32 :: from_be_bytes ([0x00 , * value . 0 , * value . 1 , * value . 2])) . is_ok () } } }
};
}
