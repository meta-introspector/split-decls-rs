// Generated macro for Buf (struct)
macro_rules! Depcrate_valueBuf {
() => {
// Module: crate::value
// Provides: {"Buf"}
// Dependencies: {}
# [derive (Debug , Clone)] struct Buf < T , const N : usize > { # [cfg (feature = "alloc")] inner : crate :: std :: boxed :: Box < [T] > , # [cfg (not (feature = "alloc"))] inner : array_vec :: ArrayVec < T , N > , }
};
}
