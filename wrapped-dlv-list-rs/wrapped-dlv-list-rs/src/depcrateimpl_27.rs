// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < T > Extend < T > for VecList < T > { fn extend < Iter > (& mut self , iter : Iter) where Iter : IntoIterator < Item = T > , { let iter = iter . into_iter () ; self . reserve (iter . size_hint () . 0) ; for value in iter { let _ = self . push_back (value) ; } } }
};
}
