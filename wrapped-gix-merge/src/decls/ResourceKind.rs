macro_rules! ResourceKind {
    () => {
        # [doc = " A way to classify the side of a resource for merging."] # [derive (Copy , Clone , Debug , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum ResourceKind { # [doc = " Our side of the state."] CurrentOrOurs , # [doc = " Their side of the state."] OtherOrTheirs , # [doc = " The state of the common base of both ours and theirs."] CommonAncestorOrBase , }
    };
}

ResourceKind!()