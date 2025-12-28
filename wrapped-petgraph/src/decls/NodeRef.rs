macro_rules! NodeRef {
    () => {
        # [doc = " A node reference."] pub trait NodeRef : Copy { type NodeId ; type Weight ; fn id (& self) -> Self :: NodeId ; fn weight (& self) -> & Self :: Weight ; }
    };
}

NodeRef!()