macro_rules! CfgList {
    () => {
        # [derive (Serialize , Deserialize , Debug , Clone , Eq , PartialEq , Default)] # [serde (transparent)] struct CfgList (# [serde (with = "cfg_")] Vec < CfgAtom >) ;
    };
}

CfgList!()