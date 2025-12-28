macro_rules! TomlLintLevel {
    () => {
        # [derive (Serialize , Deserialize , Debug , Copy , Clone , Eq , PartialEq)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlLintLevel { Forbid , Deny , Warn , Allow , }
    };
}

TomlLintLevel!();