// Generated macro for BoundedNonEmptyConnectionIdVecDeque (struct)
macro_rules! Depcrate_cidBoundedNonEmptyConnectionIdVecDeque {
() => {
// Module: crate::cid
// Provides: {"BoundedNonEmptyConnectionIdVecDeque"}
// Dependencies: {}
# [derive (Default)] struct BoundedNonEmptyConnectionIdVecDeque { # [doc = " The inner `VecDeque`."] inner : VecDeque < ConnectionIdEntry > , # [doc = " The maximum number of elements that the `VecDeque` can have."] capacity : usize , }
};
}
