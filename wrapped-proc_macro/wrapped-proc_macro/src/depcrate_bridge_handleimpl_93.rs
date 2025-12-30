// Generated macro for impl_93 (impl)
macro_rules! Depcrate_bridge_handleimpl_93 {
() => {
// Module: crate::bridge::handle
// Provides: {"impl_93"}
// Dependencies: {}
impl < T : Copy + Eq + Hash > InternedStore < T > { pub (super) fn new (counter : & 'static AtomicU32) -> Self { InternedStore { owned : OwnedStore :: new (counter) , interner : FxHashMap :: default () } } pub (super) fn alloc (& mut self , x : T) -> Handle { let owned = & mut self . owned ; * self . interner . entry (x) . or_insert_with (| | owned . alloc (x)) } pub (super) fn copy (& mut self , h : Handle) -> T { self . owned [h] } }
};
}
