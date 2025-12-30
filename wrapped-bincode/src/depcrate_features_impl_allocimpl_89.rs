// Generated macro for impl_89 (impl)
macro_rules! Depcrate_features_impl_allocimpl_89 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_89"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < Context , T > Decode < Context > for Arc < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Arc :: new (t)) } }
};
}
