macro_rules! deps {
    () => {
        NFA!();
        StateID!();
    };
}

macro_rules! Visited {
    () => {
        deps!();
        # [doc = " A bitset that keeps track of whether a particular (StateID, offset) has"] # [doc = " been considered during backtracking. If it has already been visited, then"] # [doc = " backtracking skips it. This is what gives backtracking its \"bound.\""] # [derive (Clone , Debug)] struct Visited { # [doc = " The actual underlying bitset. Each element in the bitset corresponds"] # [doc = " to a particular (StateID, offset) pair. States correspond to the rows"] # [doc = " and the offsets correspond to the columns."] # [doc = ""] # [doc = " If our underlying NFA has N states and the haystack we're searching"] # [doc = " has M bytes, then we have N*(M+1) entries in our bitset table. The"] # [doc = " M+1 occurs because our matches are delayed by one byte (to support"] # [doc = " look-around), and so we need to handle the end position itself rather"] # [doc = " than stopping just before the end. (If there is no end position, then"] # [doc = " it's treated as \"end-of-input,\" which is matched by things like '$'.)"] # [doc = ""] # [doc = " Given BITS=N*(M+1), we wind up with div_ceil(BITS, sizeof(usize))"] # [doc = " blocks."] # [doc = ""] # [doc = " We use 'usize' to represent our blocks because it makes some of the"] # [doc = " arithmetic in 'insert' a bit nicer. For example, if we used 'u32' for"] # [doc = " our block, we'd either need to cast u32s to usizes or usizes to u32s."] bitset : Vec < usize > , # [doc = " The stride represents one plus length of the haystack we're searching"] # [doc = " (as described above). The stride must be initialized for each search."] stride : usize , }
    };
}

Visited!();