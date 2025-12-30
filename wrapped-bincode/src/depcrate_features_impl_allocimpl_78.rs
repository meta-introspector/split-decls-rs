// Generated macro for impl_78 (impl)
macro_rules! Depcrate_features_impl_allocimpl_78 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_78"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Cow < '_ , T > where T : ToOwned + ? Sized , < T as ToOwned > :: Owned : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = < T as ToOwned > :: Owned :: decode (decoder) ? ; Ok (Cow :: Owned (t)) } }
};
}
