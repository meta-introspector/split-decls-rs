// Generated macro for impl_125 (impl)
macro_rules! Depcrate_ptrimpl_125 {
() => {
// Module: crate::ptr
// Provides: {"impl_125"}
// Dependencies: {}
# [doc = " Safety: RefPtr indicates an exclusive reference to a value and as such exhibits the same Send +"] # [doc = " Sync behavior of &'a mut T"] unsafe impl < 'a , T : ? Sized > Send for MutPtr < 'a , T > where & 'a mut T : Send { }
};
}
