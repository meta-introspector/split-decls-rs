// Generated macro for impl_87 (impl)
macro_rules! Depcrate_features_impl_allocimpl_87 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_87"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Rc < [T] > where T : Decode < Context > + 'static , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let vec = Vec :: decode (decoder) ? ; Ok (vec . into ()) } }
};
}
