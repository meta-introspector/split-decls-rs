// Generated macro for impl_130 (impl)
macro_rules! Depcrate_cow_mutimpl_130 {
() => {
// Module: crate::cow_mut
// Provides: {"impl_130"}
// Dependencies: {}
impl < T > std :: ops :: Deref for CowMut < '_ , T > { type Target = T ; fn deref (& self) -> & T { match self { CowMut :: Owned (it) => it , CowMut :: Borrowed (it) => it , } } }
};
}
