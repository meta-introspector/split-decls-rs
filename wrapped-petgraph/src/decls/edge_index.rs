macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndex!();
    };
}

macro_rules! edge_index {
    () => {
        deps!();
        # [doc = " Short version of `EdgeIndex::new`"] pub fn edge_index < Ix : IndexType > (index : usize) -> EdgeIndex < Ix > { EdgeIndex :: new (index) }
    };
}

edge_index!();