// Generated macro for impl_125 (impl)
macro_rules! Depcrate_parseimpl_125 {
() => {
// Module: crate::parse
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'a > Index < HeadingIndex > for Allocations < 'a > { type Output = HeadingAttributes < 'a > ; fn index (& self , ix : HeadingIndex) -> & Self :: Output { self . headings . index (ix . 0 . get () - 1) } }
};
}
