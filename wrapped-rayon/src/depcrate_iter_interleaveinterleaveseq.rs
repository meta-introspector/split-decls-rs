// Generated macro for InterleaveSeq (struct)
macro_rules! Depcrate_iter_interleaveInterleaveSeq {
() => {
// Module: crate::iter::interleave
// Provides: {"InterleaveSeq"}
// Dependencies: {}
# [doc = " Wrapper for Interleave to implement DoubleEndedIterator and"] # [doc = " ExactSizeIterator."] # [doc = ""] # [doc = " This iterator is fused."] struct InterleaveSeq < I , J > { i : Fuse < I > , j : Fuse < J > , # [doc = " Flag to control which iterator should provide the next element. When"] # [doc = " `false` then `i` produces the next element, otherwise `j` produces the"] # [doc = " next element."] i_next : bool , }
};
}
