// Generated macro for impl_30 (impl)
macro_rules! Depcrate_atomicimpl_30 {
() => {
// Module: crate::atomic
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg (target_has_atomic = "8")] impl < Context > Decode < Context > for AtomicI8 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicI8 :: new (Decode :: decode (decoder) ?)) } }
};
}
