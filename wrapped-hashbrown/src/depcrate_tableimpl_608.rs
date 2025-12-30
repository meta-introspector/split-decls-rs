// Generated macro for impl_608 (impl)
macro_rules! Depcrate_tableimpl_608 {
() => {
// Module: crate::table
// Provides: {"impl_608"}
// Dependencies: {}
impl < T , A > Iterator for IntoIter < T , A > where A : Allocator , { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , f) } }
};
}
