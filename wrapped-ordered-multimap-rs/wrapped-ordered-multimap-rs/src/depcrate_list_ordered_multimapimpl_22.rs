// Generated macro for impl_22 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_22 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_22"}
// Dependencies: {}
impl < Key , Value , State > FromIterator < (Key , Value) > for ListOrderedMultimap < Key , Value , State > where Key : Eq + Hash , State : BuildHasher + Default , { fn from_iter < Iter > (iter : Iter) -> Self where Iter : IntoIterator < Item = (Key , Value) > , { let mut map = ListOrderedMultimap :: with_hasher (State :: default ()) ; map . extend (iter) ; map } }
};
}
