// Generated macro for impl_18 (impl)
macro_rules! Depcrate_de_deserializerimpl_18 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'de , F > Deserializer < 'de , F > where F : Flavor < 'de > + 'de , { # [doc = " Obtain a Deserializer from a slice of bytes"] pub fn from_flavor (flavor : F) -> Self { Deserializer { flavor , _plt : PhantomData , } } # [doc = " Return the remaining (unused) bytes in the Deserializer along with any"] # [doc = " additional data provided by the [`Flavor`]"] pub fn finalize (self) -> Result < F :: Remainder > { self . flavor . finalize () } }
};
}
