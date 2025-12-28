macro_rules! deps {
    () => {
        TomlLintLevel!();
        TomlValueWrapper!();
    };
}

macro_rules! TomlLintConfig {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlLintConfig { pub level : TomlLintLevel , # [serde (default)] pub priority : i8 , # [serde (flatten)] # [cfg_attr (feature = "unstable-schema" , schemars (with = "HashMap<String, TomlValueWrapper>"))] pub config : toml :: Table , }
    };
}

TomlLintConfig!()