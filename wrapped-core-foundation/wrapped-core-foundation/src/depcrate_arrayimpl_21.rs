// Generated macro for impl_21 (impl)
macro_rules! Depcrate_arrayimpl_21 {
() => {
// Module: crate::array
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , T : FromVoid > Iterator for CFArrayIterator < 'a , T > { type Item = ItemRef < 'a , T > ; fn next (& mut self) -> Option < ItemRef < 'a , T > > { if self . index >= self . len { None } else { let value = unsafe { self . array . get_unchecked (self . index) } ; self . index += 1 ; Some (value) } } }
};
}
