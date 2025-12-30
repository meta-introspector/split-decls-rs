// Generated macro for impl_152 (impl)
macro_rules! Depcrateimpl_152 {
() => {
// Module: crate
// Provides: {"impl_152"}
// Dependencies: {}
impl < T , const N : usize > core :: ops :: DerefMut for SmallVec < T , N > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
};
}
