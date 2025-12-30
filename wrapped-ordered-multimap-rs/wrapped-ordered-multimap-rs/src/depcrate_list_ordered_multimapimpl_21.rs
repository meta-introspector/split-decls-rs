// Generated macro for impl_21 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_21 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , Key , Value , State > Extend < (& 'a Key , & 'a Value) > for ListOrderedMultimap < Key , Value , State > where Key : Copy + Eq + Hash , Value : Copy , State : BuildHasher , { fn extend < Iter > (& mut self , iter : Iter) where Iter : IntoIterator < Item = (& 'a Key , & 'a Value) > , { self . extend (iter . into_iter () . map (| (& key , & value) | (key , value))) ; } }
};
}
