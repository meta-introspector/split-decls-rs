// Generated macro for impl_39 (impl)
macro_rules! Depcrate_atomicimpl_39 {
() => {
// Module: crate::atomic
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (target_has_atomic = "64")] impl < Context > Decode < Context > for AtomicI64 { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicI64 :: new (Decode :: decode (decoder) ?)) } }
};
}
