// Generated macro for impl_205 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_205 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_205"}
// Dependencies: {}
impl From < Utf16Char > for Utf16Iterator { fn from (uc : Utf16Char) -> Self { let (first , second) = uc . to_tuple () ; let second = second . unwrap_or (SECOND_USED) ; Utf16Iterator { first , second } } }
};
}
