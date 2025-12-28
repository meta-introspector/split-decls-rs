macro_rules! deps {
    () => {
        Serialize!();
        Deserialize!();
    };
}

macro_rules! SubmoduleInfo {
    () => {
        deps!();
        # [cfg_attr (feature = "serde_enabled" , derive (Debug , Serialize , Deserialize))] # [cfg_attr (not (feature = "serde_enabled") , derive (Debug))] pub struct SubmoduleInfo { pub name : String , pub path : PathBuf , pub url : Option < String > , }
    };
}

SubmoduleInfo!()