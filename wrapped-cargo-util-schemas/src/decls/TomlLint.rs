macro_rules! deps {
    () => {
        TomlLintConfig!();
        TomlLintLevel!();
    };
}

macro_rules! TomlLint {
    () => {
        deps!();
        # [derive (Serialize , Debug , Clone)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlLint { Level (TomlLintLevel) , Config (TomlLintConfig) , }
    };
}

TomlLint!();