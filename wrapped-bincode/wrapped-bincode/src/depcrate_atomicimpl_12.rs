// Generated macro for impl_12 (impl)
macro_rules! Depcrate_atomicimpl_12 {
() => {
// Module: crate::atomic
// Provides: {"impl_12"}
// Dependencies: {}
# [cfg (target_has_atomic = "8")] impl < Context > Decode < Context > for AtomicBool { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicBool :: new (Decode :: decode (decoder) ?)) } }
};
}
