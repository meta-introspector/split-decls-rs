// Generated macro for impl_106 (impl)
macro_rules! Depcrate_provider_exceptions_builderimpl_106 {
() => {
// Module: crate::provider::exceptions_builder
// Provides: {"impl_106"}
// Dependencies: {}
impl ExceptionHeader { # [doc = " Construct from an ICU4C-format u16."] pub (crate) fn from_integer (int : u16) -> Self { let slot_presence = SlotPresence (u8 :: try_from (int & ExceptionHeaderULE :: SLOTS_MASK) . unwrap_or (0)) ; let bits = ExceptionBits :: from_integer (u8 :: try_from (int >> ExceptionHeaderULE :: BITS_SHIFT) . unwrap_or (0) ,) ; Self { slot_presence , bits , } } pub (crate) fn has_slot (self , slot : ExceptionSlot) -> bool { self . slot_presence . has_slot (slot) } }
};
}
