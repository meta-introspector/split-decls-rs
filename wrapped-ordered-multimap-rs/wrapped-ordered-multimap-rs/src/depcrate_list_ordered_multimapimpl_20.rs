// Generated macro for impl_20 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_20 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_20"}
// Dependencies: {}
impl < Key , Value , State > Extend < (Key , Value) > for ListOrderedMultimap < Key , Value , State > where Key : Eq + Hash , State : BuildHasher , { fn extend < Iter > (& mut self , iter : Iter) where Iter : IntoIterator < Item = (Key , Value) > , { let iter = iter . into_iter () ; self . reserve_values (iter . size_hint () . 0) ; for (key , value) in iter { let _ = self . append (key , value) ; } } }
};
}
