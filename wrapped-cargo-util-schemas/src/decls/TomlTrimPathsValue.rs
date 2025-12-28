macro_rules! TomlTrimPathsValue {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq , Ord , PartialOrd , Hash , Serialize , Deserialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlTrimPathsValue { Diagnostics , Macro , Object , }
    };
}

TomlTrimPathsValue!()