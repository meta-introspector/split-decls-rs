// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl ExactSizeIterator for BytesToHexChars < '_ > { fn len (& self) -> usize { let mut length = self . inner . len () * 2 ; if self . next . is_some () { length += 1 ; } length } }
};
}
