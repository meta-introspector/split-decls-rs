// Generated macro for impl_61 (impl)
macro_rules! Depcrate_providerimpl_61 {
() => {
// Module: crate::provider
// Provides: {"impl_61"}
// Dependencies: {}
impl FourBitMetadata { # [doc = " Creates a [`FourBitMetadata`] if the given value fits in 4 bits."] pub fn try_from_byte (byte : u8) -> Option < Self > { if byte < 0x80 { Some (Self (byte)) } else { None } } # [doc = " Creates a [`FourBitMetadata`] with a zero value."] pub fn zero () -> Self { Self (0) } # [doc = " Gets the value out of a [`FourBitMetadata`]."] pub fn get (self) -> u8 { self . 0 } }
};
}
