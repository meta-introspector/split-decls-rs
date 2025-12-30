// Generated macro for impl_132 (impl)
macro_rules! Depcrate_rawimpl_132 {
() => {
// Module: crate::raw
// Provides: {"impl_132"}
// Dependencies: {}
impl < T > RawIterHash < T > { # [cfg_attr (feature = "inline-more" , inline)] unsafe fn new < A : Allocator > (table : & RawTable < T , A > , hash : u64) -> Self { RawIterHash { inner : RawIterHashInner :: new (& table . table , hash) , _marker : PhantomData , } } }
};
}
