// Generated macro for impl_14 (impl)
macro_rules! Depcrate_ucharimpl_14 {
() => {
// Module: crate::uchar
// Provides: {"impl_14"}
// Dependencies: {}
impl PartialOrd < PotentialCodePoint > for char { fn partial_cmp (& self , other : & PotentialCodePoint) -> Option < Ordering > { PotentialCodePoint :: from_char (* self) . partial_cmp (other) } }
};
}
