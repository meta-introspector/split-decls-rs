// Generated macro for impl_63 (impl)
macro_rules! Depcrate_cacheimpl_63 {
() => {
// Module: crate::cache
// Provides: {"impl_63"}
// Dependencies: {}
impl < A , T , S > From < A > for Cache < A , T > where A : Deref < Target = ArcSwapAny < T , S > > , T : RefCnt , S : Strategy < T > , { fn from (arc_swap : A) -> Self { Self :: new (arc_swap) } }
};
}
