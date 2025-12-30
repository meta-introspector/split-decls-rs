// Generated macro for impl_21 (impl)
macro_rules! Depcrate_atomicimpl_21 {
() => {
// Module: crate::atomic
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (target_has_atomic = "32")] impl < Context > Decode < Context > for AtomicU32 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicU32 :: new (Decode :: decode (decoder) ?)) } }
};
}
