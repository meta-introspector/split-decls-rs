macro_rules! Undirected {
    () => {
        # [doc = " Marker type for an undirected graph."] # [derive (Clone , Copy , Debug)] # [cfg_attr (feature = "serde-1" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub enum Undirected { }
    };
}

Undirected!();