// Generated macro for Cache (struct)
macro_rules! Depcrate_cacheCache {
() => {
// Module: crate::cache
// Provides: {"Cache"}
// Dependencies: {}
pub struct Cache < I , R > where I : Iterator , { iter : RefCell < I > , items : UnsafeCell < ChunkyVec < I :: Item > > , res : std :: marker :: PhantomData < R > , }
};
}
