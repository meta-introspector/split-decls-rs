// Generated macro for impl_76 (impl)
macro_rules! Depcrate_atomicimpl_76 {
() => {
// Module: crate::atomic
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : ? Sized + Pointable > Ord for Shared < '_ , T > { fn cmp (& self , other : & Self) -> cmp :: Ordering { self . data . cmp (& other . data) } }
};
}
