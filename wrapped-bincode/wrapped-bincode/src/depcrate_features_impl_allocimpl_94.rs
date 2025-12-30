// Generated macro for impl_94 (impl)
macro_rules! Depcrate_features_impl_allocimpl_94 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < Context , T > Decode < Context > for Arc < [T] > where T : Decode < Context > + 'static , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let vec = Vec :: decode (decoder) ? ; Ok (vec . into ()) } }
};
}
