// Generated macro for impl_60 (impl)
macro_rules! Depcrate_ensureimpl_60 {
() => {
// Module: crate::ensure
// Provides: {"impl_60"}
// Dependencies: {}
impl Write for Buf { fn write_str (& mut self , s : & str) -> fmt :: Result { if s . bytes () . any (| b | b == b' ' || b == b'\n') { return Err (fmt :: Error) ; } let remaining = self . bytes . len () - self . written ; if s . len () > remaining { return Err (fmt :: Error) ; } unsafe { ptr :: copy_nonoverlapping (s . as_ptr () , self . bytes . as_mut_ptr () . add (self . written) . cast :: < u8 > () , s . len () ,) ; } self . written += s . len () ; Ok (()) } }
};
}
