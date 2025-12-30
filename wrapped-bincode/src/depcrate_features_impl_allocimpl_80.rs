// Generated macro for impl_80 (impl)
macro_rules! Depcrate_features_impl_allocimpl_80 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_80"}
// Dependencies: {}
impl < T > Encode for Cow < '_ , T > where T : ToOwned + ? Sized , for < 'a > & 'a T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_ref () . encode (encoder) } }
};
}
