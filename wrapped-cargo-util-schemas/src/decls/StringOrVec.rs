macro_rules! StringOrVec {
    () => {
        # [doc = " This can be parsed from either a TOML string or array,"] # [doc = " but is always stored as a vector."] # [derive (Clone , Debug , Serialize , Eq , PartialEq , PartialOrd , Ord)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct StringOrVec (pub Vec < String >) ;
    };
}

StringOrVec!();