// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl IterState { fn new < T > (slab : & LruSlab < T >) -> Self { Self { head : slab . head , tail : slab . tail , len : slab . len , } } fn next (& mut self , get_next : impl Fn (u32) -> u32) -> Option < u32 > { if self . len == 0 { return None ; } let idx = self . head ; self . head = get_next (idx) ; self . len -= 1 ; Some (idx) } fn next_back (& mut self , get_prev : impl Fn (u32) -> u32) -> Option < u32 > { if self . len == 0 { return None ; } let idx = self . tail ; self . tail = get_prev (idx) ; self . len -= 1 ; Some (idx) } }
};
}
