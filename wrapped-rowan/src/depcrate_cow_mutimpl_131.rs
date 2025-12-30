// Generated macro for impl_131 (impl)
macro_rules! Depcrate_cow_mutimpl_131 {
() => {
// Module: crate::cow_mut
// Provides: {"impl_131"}
// Dependencies: {}
impl < T > std :: ops :: DerefMut for CowMut < '_ , T > { fn deref_mut (& mut self) -> & mut T { match self { CowMut :: Owned (it) => it , CowMut :: Borrowed (it) => it , } } }
};
}
