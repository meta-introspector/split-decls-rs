macro_rules! deps {
    () => {
        LocalizedNode!();
    };
}

macro_rules! LocalizedConstraintGraph {
    () => {
        deps!();
        # [doc = " The localized constraint graph indexes the physical and logical edges to compute a given node's"] # [doc = " successors during traversal."] struct LocalizedConstraintGraph { # [doc = " The actual, physical, edges we have recorded for a given node."] edges : FxHashMap < LocalizedNode , FxIndexSet < LocalizedNode > > , # [doc = " The logical edges representing the outlives constraints that hold at all points in the CFG,"] # [doc = " which we don't localize to avoid creating a lot of unnecessary edges in the graph. Some CFGs"] # [doc = " can be big, and we don't need to create such a physical edge for every point in the CFG."] logical_edges : FxHashMap < RegionVid , FxIndexSet < RegionVid > > , }
    };
}

LocalizedConstraintGraph!();