// Generated macro for impl_429 (impl)
macro_rules! Depcrate_util_prefilterimpl_429 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_429"}
// Dependencies: {}
impl core :: fmt :: Debug for RareByteOffsets { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut offsets = vec ! [] ; for off in self . set . iter () { if off . max > 0 { offsets . push (off) ; } } f . debug_struct ("RareByteOffsets") . field ("set" , & offsets) . finish () } }
};
}
