macro_rules! WorkspaceValue {
    () => {
        # [derive (Deserialize , Serialize , Copy , Clone , Debug)] # [serde (try_from = "bool")] # [serde (into = "bool")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] struct WorkspaceValue ;
    };
}

WorkspaceValue!()