macro_rules! FullName {
    () => {
        # [doc = " A validated complete and fully qualified reference name, safe to use for all operations."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct FullName (pub (crate) BString) ;
    };
}

FullName!();