// Generated macro for impl_309 (impl)
macro_rules! Depcrate_attrimpl_309 {
() => {
// Module: crate::attr
// Provides: {"impl_309"}
// Dependencies: {}
impl MarkedAttrs { pub fn new () -> Self { MarkedAttrs (GrowableBitSet :: new_empty ()) } pub fn mark (& mut self , attr : & Attribute) { self . 0 . insert (attr . id) ; } pub fn is_marked (& self , attr : & Attribute) -> bool { self . 0 . contains (attr . id) } }
};
}
