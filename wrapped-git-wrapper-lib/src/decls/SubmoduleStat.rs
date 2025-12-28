macro_rules! deps {
    () => {
        Deserialize!();
        Serialize!();
    };
}

macro_rules! SubmoduleStat {
    () => {
        deps!();
        # [cfg_attr (feature = "serde_enabled" , derive (Debug , Serialize , Deserialize , PartialEq , Eq , Clone))] # [cfg_attr (not (feature = "serde_enabled") , derive (Debug , PartialEq , Eq , Clone))] pub struct SubmoduleStat { pub head_commit : String , pub workdir_hash : String , }
    };
}

SubmoduleStat!()