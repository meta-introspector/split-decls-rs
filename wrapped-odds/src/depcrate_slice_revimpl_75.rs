// Generated macro for impl_75 (impl)
macro_rules! Depcrate_slice_revimpl_75 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_75"}
// Dependencies: {}
# [doc = " `RevSlice` compares by logical element sequence."] impl < T , U > PartialEq < [U] > for RevSlice < T > where T : PartialEq < U > , { fn eq (& self , rhs : & [U]) -> bool { if self . len () != rhs . len () { return false ; } for (x , y) in self . into_iter () . zip (rhs) { if x != y { return false ; } } true } }
};
}
