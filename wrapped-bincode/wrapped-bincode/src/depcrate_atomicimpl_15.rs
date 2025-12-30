// Generated macro for impl_15 (impl)
macro_rules! Depcrate_atomicimpl_15 {
() => {
// Module: crate::atomic
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (target_has_atomic = "8")] impl < Context > Decode < Context > for AtomicU8 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicU8 :: new (Decode :: decode (decoder) ?)) } }
};
}
