// Generated macro for SparseSets (struct)
macro_rules! Depcrate_util_sparse_setSparseSets {
() => {
// Module: crate::util::sparse_set
// Provides: {"SparseSets"}
// Dependencies: {}
# [doc = " A pairse of sparse sets."] # [doc = ""] # [doc = " This is useful when one needs to compute NFA epsilon closures from a"] # [doc = " previous set of states derived from an epsilon closure. One set can be the"] # [doc = " starting states where as the other set can be the destination states after"] # [doc = " following the transitions for a particular byte of input."] # [doc = ""] # [doc = " There is no significance to 'set1' or 'set2'. They are both sparse sets of"] # [doc = " the same size."] # [doc = ""] # [doc = " The members of this struct are exposed so that callers may borrow 'set1'"] # [doc = " and 'set2' individually without being force to borrow both at the same"] # [doc = " time."] # [derive (Clone , Debug)] pub (crate) struct SparseSets { pub (crate) set1 : SparseSet , pub (crate) set2 : SparseSet , }
};
}
