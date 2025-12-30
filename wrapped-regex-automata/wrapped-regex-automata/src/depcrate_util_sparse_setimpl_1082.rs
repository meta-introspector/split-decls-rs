// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_util_sparse_setimpl_1082 {
() => {
// Module: crate::util::sparse_set
// Provides: {"impl_1082"}
// Dependencies: {}
impl core :: fmt :: Debug for SparseSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let elements : Vec < StateID > = self . iter () . collect () ; f . debug_tuple ("SparseSet") . field (& elements) . finish () } }
};
}
