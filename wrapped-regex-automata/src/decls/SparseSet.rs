macro_rules! deps {
    () => {
        NFA!();
        StateID!();
    };
}

macro_rules! SparseSet {
    () => {
        deps!();
        # [doc = " A sparse set used for representing ordered NFA states."] # [doc = ""] # [doc = " This supports constant time addition and membership testing. Clearing an"] # [doc = " entire set can also be done in constant time. Iteration yields elements"] # [doc = " in the order in which they were inserted."] # [doc = ""] # [doc = " The data structure is based on: https://research.swtch.com/sparse"] # [doc = " Note though that we don't actually use uninitialized memory. We generally"] # [doc = " reuse sparse sets, so the initial allocation cost is bearable. However, its"] # [doc = " other properties listed above are extremely useful."] # [derive (Clone)] pub (crate) struct SparseSet { # [doc = " The number of elements currently in this set."] len : usize , # [doc = " Dense contains the ids in the order in which they were inserted."] dense : Vec < StateID > , # [doc = " Sparse maps ids to their location in dense."] # [doc = ""] # [doc = " A state ID is in the set if and only if"] # [doc = " sparse[id] < len && id == dense[sparse[id]]."] # [doc = ""] # [doc = " Note that these are indices into 'dense'. It's a little weird to use"] # [doc = " StateID here, but we know our length can never exceed the bounds of"] # [doc = " StateID (enforced by 'resize') and StateID will be at most 4 bytes"] # [doc = " where as a usize is likely double that in most cases."] sparse : Vec < StateID > , }
    };
}

SparseSet!();