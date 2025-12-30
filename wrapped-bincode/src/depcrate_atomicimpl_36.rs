// Generated macro for impl_36 (impl)
macro_rules! Depcrate_atomicimpl_36 {
() => {
// Module: crate::atomic
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg (target_has_atomic = "32")] impl < Context > Decode < Context > for AtomicI32 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicI32 :: new (Decode :: decode (decoder) ?)) } }
};
}
