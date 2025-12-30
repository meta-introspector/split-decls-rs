// Generated macro for impl_27 (impl)
macro_rules! Depcrate_atomicimpl_27 {
() => {
// Module: crate::atomic
// Provides: {"impl_27"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < Context > Decode < Context > for AtomicUsize { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { Ok (AtomicUsize :: new (Decode :: decode (decoder) ?)) } }
};
}
