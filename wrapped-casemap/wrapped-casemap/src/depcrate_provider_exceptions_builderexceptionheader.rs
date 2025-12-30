// Generated macro for ExceptionHeader (struct)
macro_rules! Depcrate_provider_exceptions_builderExceptionHeader {
() => {
// Module: crate::provider::exceptions_builder
// Provides: {"ExceptionHeader"}
// Dependencies: {}
# [doc = " The header for exception types as found in ICU4C data. See [`ExceptionHeaderULE`]"] # [doc = " for the wire format"] # [derive (Copy , Clone , PartialEq , Eq)] pub struct ExceptionHeader { # [doc = " The various slots that are present, masked by ExceptionSlot"] # [doc = ""] # [doc = " We still store this as a bitmask since it's more convenient to access as one"] pub slot_presence : SlotPresence , pub bits : ExceptionBits , }
};
}
