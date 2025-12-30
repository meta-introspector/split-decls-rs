// Generated macro for impl_66 (impl)
macro_rules! Depcrate_provider_exception_helpersimpl_66 {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"impl_66"}
// Dependencies: {}
impl SlotPresence { pub (crate) fn add_slot (& mut self , slot : ExceptionSlot) { self . 0 |= 1 << slot as u8 ; } pub (crate) fn has_slot (self , slot : ExceptionSlot) -> bool { let bit = 1 << (slot as u8) ; self . 0 & bit != 0 } }
};
}
