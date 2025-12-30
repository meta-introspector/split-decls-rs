// Generated macro for impl_180 (impl)
macro_rules! Depcrateimpl_180 {
() => {
// Module: crate
// Provides: {"impl_180"}
// Dependencies: {}
impl < T , const N : usize > Ord for SmallVec < T , N > where T : Ord , { # [inline] fn cmp (& self , other : & SmallVec < T , N >) -> core :: cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
};
}
