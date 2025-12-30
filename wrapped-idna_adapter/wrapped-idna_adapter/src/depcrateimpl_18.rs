// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl JoiningType { # [doc = " Returns the corresponding `JoiningTypeMask`."] # [inline (always)] pub fn to_mask (self) -> JoiningTypeMask { JoiningTypeMask (joining_type_to_mask (self . 0)) } # [inline (always)] pub fn is_transparent (self) -> bool { self . 0 == icu_properties :: props :: JoiningType :: Transparent } }
};
}
