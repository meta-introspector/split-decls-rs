macro_rules! ResourceKind {
    () => {
        # [doc = " A way to classify a resource suitable for diffing."] # [derive (Copy , Clone , Debug , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum ResourceKind { # [doc = " The source of a rewrite, rename or copy operation, or generally the old version of a resource."] OldOrSource , # [doc = " The destination of a rewrite, rename or copy operation, or generally the new version of a resource."] NewOrDestination , }
    };
}

ResourceKind!()