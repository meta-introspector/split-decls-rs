macro_rules! deps {
    () => {
        Undirected!();
        Directed!();
    };
}

macro_rules! EdgeProperty {
    () => {
        deps!();
        # [derive (Serialize , Deserialize)] # [serde (rename_all = "lowercase")] # [derive (Debug)] pub enum EdgeProperty { Undirected , Directed , }
    };
}

EdgeProperty!();