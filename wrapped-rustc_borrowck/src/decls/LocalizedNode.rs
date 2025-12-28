macro_rules! LocalizedNode {
    () => {
        # [doc = " A node in the graph to be traversed, one of the two vertices of a localized outlives constraint."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] struct LocalizedNode { region : RegionVid , point : PointIndex , }
    };
}

LocalizedNode!();