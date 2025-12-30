// Generated macro for impl_119 (impl)
macro_rules! Depcrate_ptrimpl_119 {
() => {
// Module: crate::ptr
// Provides: {"impl_119"}
// Dependencies: {}
# [doc = " Safety: RefPtr indicates a shared reference to a value and as such exhibits the same Send +"] # [doc = " Sync behavior of &'a T"] unsafe impl < 'a , T : ? Sized > Send for RefPtr < 'a , T > where & 'a T : Send { }
};
}
