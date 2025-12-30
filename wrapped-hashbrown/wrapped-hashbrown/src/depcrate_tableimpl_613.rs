// Generated macro for impl_613 (impl)
macro_rules! Depcrate_tableimpl_613 {
() => {
// Module: crate::table
// Provides: {"impl_613"}
// Dependencies: {}
impl < T , A : Allocator > Iterator for Drain < '_ , T , A > { type Item = T ; fn next (& mut self) -> Option < T > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , f) } }
};
}
