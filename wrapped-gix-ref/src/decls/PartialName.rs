macro_rules! PartialName {
    () => {
        # [doc = " A validated and potentially partial reference name, safe to use for common operations."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct PartialName (BString) ;
    };
}

PartialName!()