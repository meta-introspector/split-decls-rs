macro_rules! Version {
    () => {
        # [doc = " Known multi-index file versions"] # [derive (Default , PartialEq , Eq , Ord , PartialOrd , Debug , Hash , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (missing_docs)] pub enum Version { # [default] V1 = 1 , }
    };
}

Version!()