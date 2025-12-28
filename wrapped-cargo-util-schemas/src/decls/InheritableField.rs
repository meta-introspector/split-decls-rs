macro_rules! deps {
    () => {
        TomlInheritedField!();
    };
}

macro_rules! InheritableField {
    () => {
        deps!();
        # [doc = " An enum that allows for inheriting keys from a workspace in a Cargo.toml."] # [derive (Serialize , Copy , Clone , Debug)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum InheritableField < T > { # [doc = " The type that is used when not inheriting from a workspace."] Value (T) , # [doc = " The type when inheriting from a workspace."] Inherit (TomlInheritedField) , }
    };
}

InheritableField!()