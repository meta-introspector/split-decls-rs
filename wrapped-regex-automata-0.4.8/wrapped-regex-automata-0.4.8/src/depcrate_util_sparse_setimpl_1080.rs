// Generated macro for impl_1080 (impl)
macro_rules! Depcrate_util_sparse_setimpl_1080 {
() => {
// Module: crate::util::sparse_set
// Provides: {"impl_1080"}
// Dependencies: {}
impl core :: fmt :: Debug for SparseSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let elements : Vec < StateID > = self . iter () . collect () ; f . debug_tuple ("SparseSet") . field (& elements) . finish () } }
};
}
