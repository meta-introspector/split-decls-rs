// Generated macro for impl_65 (impl)
macro_rules! Depcrate_cacheimpl_65 {
() => {
// Module: crate::cache
// Provides: {"impl_65"}
// Dependencies: {}
impl < A , T , S , F , U > Access < U > for MapCache < A , T , F > where A : Deref < Target = ArcSwapAny < T , S > > , T : RefCnt , S : Strategy < T > , F : FnMut (& T) -> & U , { fn load (& mut self) -> & U { (self . projection) (self . inner . load ()) } }
};
}
