// Generated macro for impl_42 (impl)
macro_rules! Depcrate_atomicimpl_42 {
() => {
// Module: crate::atomic
// Provides: {"impl_42"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < Context > Decode < Context > for AtomicIsize { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicIsize :: new (Decode :: decode (decoder) ?)) } }
};
}
