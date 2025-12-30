// Generated macro for CustomData (trait)
macro_rules! Depcrate_data_providerCustomData {
() => {
// Module: crate::data_provider
// Provides: {"CustomData"}
// Dependencies: {}
# [doc = " Encapsulates custom data that can be wrapped."] pub trait CustomData { # [doc = " Returns a pointer to the start of the custom data. This pointer *must not change* during"] # [doc = " the lifespan of this `CustomData`."] unsafe fn ptr (& self) -> * const u8 ; # [doc = " Returns the length of this custom data. This value must not change during the lifespan of"] # [doc = " this `CustomData`."] unsafe fn len (& self) -> usize ; }
};
}
