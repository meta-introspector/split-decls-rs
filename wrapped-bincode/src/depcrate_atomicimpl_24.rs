// Generated macro for impl_24 (impl)
macro_rules! Depcrate_atomicimpl_24 {
() => {
// Module: crate::atomic
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (target_has_atomic = "64")] impl < Context > Decode < Context > for AtomicU64 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicU64 :: new (Decode :: decode (decoder) ?)) } }
};
}
