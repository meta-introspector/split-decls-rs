// Generated macro for impl_488 (impl)
macro_rules! Depcrate_setimpl_488 {
() => {
// Module: crate::set
// Provides: {"impl_488"}
// Dependencies: {}
impl < T , S , A > From < HashMap < T , () , S , A > > for HashSet < T , S , A > where A : Allocator , { fn from (map : HashMap < T , () , S , A >) -> Self { Self { map } } }
};
}
