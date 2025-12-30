// Generated macro for SparseSet (struct)
macro_rules! Depcrate_sparseSparseSet {
() => {
// Module: crate::sparse
// Provides: {"SparseSet"}
// Dependencies: {}
# [doc = " A sparse set used for representing ordered NFA states."] # [doc = ""] # [doc = " This supports constant time addition and membership testing. Clearing an"] # [doc = " entire set can also be done in constant time. Iteration yields elements"] # [doc = " in the order in which they were inserted."] # [doc = ""] # [doc = " The data structure is based on: http://research.swtch.com/sparse"] # [doc = " Note though that we don't actually use unitialized memory. We generally"] # [doc = " reuse allocations, so the initial allocation cost is bareable. However,"] # [doc = " its other properties listed above are extremely useful."] # [derive (Clone , Debug)] pub struct SparseSet { # [doc = " Dense contains the instruction pointers in the order in which they"] # [doc = " were inserted. Accessing elements >= self.size is illegal."] dense : Vec < usize > , # [doc = " Sparse maps instruction pointers to their location in dense."] # [doc = ""] # [doc = " An instruction pointer is in the set if and only if"] # [doc = " sparse[ip] < size && ip == dense[sparse[ip]]."] sparse : Vec < usize > , # [doc = " The number of elements in the set."] size : usize , }
};
}
