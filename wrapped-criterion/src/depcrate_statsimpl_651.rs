// Generated macro for impl_651 (impl)
macro_rules! Depcrate_statsimpl_651 {
() => {
// Module: crate::stats
// Provides: {"impl_651"}
// Dependencies: {}
impl < A > Deref for Distribution < A > { type Target = Sample < A > ; fn deref (& self) -> & Sample < A > { let slice : & [_] = & self . 0 ; unsafe { mem :: transmute (slice) } } }
};
}
