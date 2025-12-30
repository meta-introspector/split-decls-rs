// Generated macro for impl_33 (impl)
macro_rules! Depcrate_atomicimpl_33 {
() => {
// Module: crate::atomic
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (target_has_atomic = "16")] impl < Context > Decode < Context > for AtomicI16 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicI16 :: new (Decode :: decode (decoder) ?)) } }
};
}
