// Generated macro for impl_485 (impl)
macro_rules! Depcrate_setimpl_485 {
() => {
// Module: crate::set
// Provides: {"impl_485"}
// Dependencies: {}
impl < T , S , A > PartialEq for HashSet < T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { fn eq (& self , other : & Self) -> bool { if self . len () != other . len () { return false ; } self . iter () . all (| key | other . contains (key)) } }
};
}
