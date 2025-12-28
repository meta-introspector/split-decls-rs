macro_rules! Trust {
    () => {
        # [doc = " A way to specify how 'safe' we feel about a resource, typically about a git repository."] # [derive (Copy , Clone , Ord , PartialOrd , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Trust { # [doc = " Caution is warranted when using the resource."] Reduced , # [doc = " We have no doubts that this resource means no harm and it can be used at will."] Full , }
    };
}

Trust!()