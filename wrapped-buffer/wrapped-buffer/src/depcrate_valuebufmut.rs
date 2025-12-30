// Generated macro for BufMut (struct)
macro_rules! Depcrate_valueBufMut {
() => {
// Module: crate::value
// Provides: {"BufMut"}
// Dependencies: {}
# [derive (Debug , Clone)] struct BufMut < T , const N : usize > { # [cfg (feature = "alloc")] inner : crate :: std :: vec :: Vec < T > , # [cfg (not (feature = "alloc"))] inner : array_vec :: ArrayVec < T , N > , }
};
}
