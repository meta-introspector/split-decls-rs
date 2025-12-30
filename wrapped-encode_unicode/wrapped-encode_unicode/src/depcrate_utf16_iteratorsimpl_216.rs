// Generated macro for impl_216 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_216 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'a > Iterator for Utf16CharIndices < 'a > { type Item = (usize , Utf16Char) ; fn next (& mut self) -> Option < (usize , Utf16Char) > { match Utf16Char :: from_str_start (& self . str [self . index ..]) { Ok ((u16c , bytes)) => { let item = (self . index , u16c) ; self . index += bytes ; Some (item) } , Err (EmptyStrError) => None } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . str . len () - self . index ; (len . wrapping_add (3) / 4 , Some (len)) } }
};
}
