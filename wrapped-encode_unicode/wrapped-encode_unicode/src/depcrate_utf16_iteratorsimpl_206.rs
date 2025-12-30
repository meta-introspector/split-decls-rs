// Generated macro for impl_206 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_206 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_206"}
// Dependencies: {}
impl Iterator for Utf16Iterator { type Item = u16 ; fn next (& mut self) -> Option < u16 > { match (self . first , self . second) { (FIRST_USED , SECOND_USED) => { None } , (FIRST_USED , second) => { self . second = SECOND_USED ; Some (second) } , (first , _) => { self . first = FIRST_USED ; Some (first) } , } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
