// Generated macro for impl_696 (impl)
macro_rules! Depcrate_util_alphabetimpl_696 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_696"}
// Dependencies: {}
impl core :: fmt :: Debug for BitSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut fmtd = f . debug_set () ; for b in 0u8 ..= 255 { if (ByteSet { bits : * self }) . contains (b) { fmtd . entry (& b) ; } } fmtd . finish () } }
};
}
