// Generated macro for impl_151 (impl)
macro_rules! Depcrateimpl_151 {
() => {
// Module: crate
// Provides: {"impl_151"}
// Dependencies: {}
impl < T , const N : usize > core :: ops :: Deref for SmallVec < T , N > { type Target = [T] ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }
};
}
