// Generated macro for impl_132 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_132 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_132"}
// Dependencies: {}
impl < 'a > Iterator for Utf8CharIndices < 'a > { type Item = (usize , Utf8Char) ; fn next (& mut self) -> Option < (usize , Utf8Char) > { match Utf8Char :: from_str_start (& self . str [self . index ..]) { Ok ((u8c , len)) => { let item = (self . index , u8c) ; self . index += len ; Some (item) } , Err (EmptyStrError) => None } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . str . len () - self . index ; (len . wrapping_add (3) / 4 , Some (len)) } }
};
}
