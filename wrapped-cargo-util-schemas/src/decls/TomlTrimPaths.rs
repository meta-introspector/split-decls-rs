macro_rules! deps {
    () => {
        TomlTrimPathsValue!();
    };
}

macro_rules! TomlTrimPaths {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq , Ord , PartialOrd , Hash , Serialize)] # [serde (untagged , rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlTrimPaths { Values (Vec < TomlTrimPathsValue >) , All , }
    };
}

TomlTrimPaths!();