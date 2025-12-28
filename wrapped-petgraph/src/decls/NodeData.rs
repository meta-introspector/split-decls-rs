macro_rules! NodeData {
    () => {
        # [derive (Copy , Clone , Debug)] struct NodeData { rootindex : Option < NonZeroUsize > , }
    };
}

NodeData!()