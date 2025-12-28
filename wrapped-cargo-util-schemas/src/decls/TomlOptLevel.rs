macro_rules! TomlOptLevel {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlOptLevel (pub String) ;
    };
}

TomlOptLevel!();