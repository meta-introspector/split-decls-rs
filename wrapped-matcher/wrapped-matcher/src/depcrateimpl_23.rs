// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl std :: fmt :: Debug for BitSet { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut fmtd = f . debug_set () ; for b in 0 ..= 255 { if ByteSet (* self) . contains (b) { fmtd . entry (& b) ; } } fmtd . finish () } }
};
}
