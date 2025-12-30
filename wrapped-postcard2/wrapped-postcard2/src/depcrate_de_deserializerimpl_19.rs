// Generated macro for impl_19 (impl)
macro_rules! Depcrate_de_deserializerimpl_19 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'de > Deserializer < 'de , Slice < 'de > > { # [doc = " Obtain a Deserializer from a slice of bytes"] pub fn from_bytes (input : & 'de [u8]) -> Self { Deserializer { flavor : Slice :: new (input) , _plt : PhantomData , } } }
};
}
