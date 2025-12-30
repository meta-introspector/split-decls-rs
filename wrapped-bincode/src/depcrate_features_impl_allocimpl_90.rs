// Generated macro for impl_90 (impl)
macro_rules! Depcrate_features_impl_allocimpl_90 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < Context > Decode < Context > for Arc < str > { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let decoded = String :: decode (decoder) ? ; Ok (decoded . into ()) } }
};
}
