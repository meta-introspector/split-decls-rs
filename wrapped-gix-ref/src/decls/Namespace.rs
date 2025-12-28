macro_rules! Namespace {
    () => {
        # [doc = " A _validated_ prefix for references to act as a namespace."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Namespace (BString) ;
    };
}

Namespace!()