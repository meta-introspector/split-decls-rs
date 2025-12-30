// Generated macro for Slice (struct)
macro_rules! Depcrate_de_flavorsSlice {
() => {
// Module: crate::de::flavors
// Provides: {"Slice"}
// Dependencies: {}
# [doc = " A simple [`Flavor`] representing the deserialization from a borrowed slice"] pub struct Slice < 'de > { pub (crate) cursor : * const u8 , pub (crate) end : * const u8 , pub (crate) _pl : PhantomData < & 'de [u8] > , }
};
}
