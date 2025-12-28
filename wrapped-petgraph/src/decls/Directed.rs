macro_rules! Directed {
    () => {
        # [doc = " Marker type for a directed graph."] # [derive (Clone , Copy , Debug)] # [cfg_attr (feature = "serde-1" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub enum Directed { }
    };
}

Directed!()