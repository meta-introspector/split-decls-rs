// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < T > FromIterator < T > for VecList < T > { fn from_iter < Iter > (iter : Iter) -> Self where Iter : IntoIterator < Item = T > , { let mut list = VecList :: new () ; list . extend (iter) ; list } }
};
}
