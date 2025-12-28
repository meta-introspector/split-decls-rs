macro_rules! Name {
    () => {
        # [doc = " Represents a validated attribute name"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Name (pub (crate) KString) ;
    };
}

Name!()