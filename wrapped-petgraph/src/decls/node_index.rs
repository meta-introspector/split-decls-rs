macro_rules! deps {
    () => {
        NodeIndex!();
        DefaultIx!();
    };
}

macro_rules! node_index {
    () => {
        deps!();
        # [doc = " Short version of `NodeIndex::new` (with Ix = `DefaultIx`)"] # [inline] pub fn node_index (ax : usize) -> NodeIndex { NodeIndex :: new (ax) }
    };
}

node_index!()