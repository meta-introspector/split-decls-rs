// Generated macro for impl_350 (impl)
macro_rules! Depcrate_rsliceimpl_350 {
() => {
// Module: crate::rslice
// Provides: {"impl_350"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for rustls_str < 'a > { type Error = NulByte ; fn try_from (s : & str) -> Result < Self , Self :: Error > { if s . contains ('\0') { return Err (NulByte { }) ; } Ok (rustls_str { data : s . as_ptr () as * const c_char , len : s . len () , phantom : PhantomData , }) } }
};
}
