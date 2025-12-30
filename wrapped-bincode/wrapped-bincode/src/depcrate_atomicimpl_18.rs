// Generated macro for impl_18 (impl)
macro_rules! Depcrate_atomicimpl_18 {
() => {
// Module: crate::atomic
// Provides: {"impl_18"}
// Dependencies: {}
# [cfg (target_has_atomic = "16")] impl < Context > Decode < Context > for AtomicU16 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicU16 :: new (Decode :: decode (decoder) ?)) } }
};
}
