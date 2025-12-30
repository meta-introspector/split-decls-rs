// Generated macro for impl_62 (impl)
macro_rules! Depcrate_cacheimpl_62 {
() => {
// Module: crate::cache
// Provides: {"impl_62"}
// Dependencies: {}
impl < A , T , S > Access < T :: Target > for Cache < A , T > where A : Deref < Target = ArcSwapAny < T , S > > , T : Deref < Target = < T as RefCnt > :: Base > + RefCnt , S : Strategy < T > , { fn load (& mut self) -> & T :: Target { self . load () . deref () } }
};
}
