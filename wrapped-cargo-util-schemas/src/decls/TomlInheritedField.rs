macro_rules! deps {
    () => {
        WorkspaceValue!();
    };
}

macro_rules! TomlInheritedField {
    () => {
        deps!();
        # [derive (Deserialize , Serialize , Copy , Clone , Debug)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlInheritedField { workspace : WorkspaceValue , }
    };
}

TomlInheritedField!()