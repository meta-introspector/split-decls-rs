macro_rules! deps {
    () => {
        TomlDependency!();
        TomlInheritedDependency!();
    };
}

macro_rules! InheritableDependency {
    () => {
        deps!();
        # [derive (Serialize , Clone , Debug)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum InheritableDependency { # [doc = " The type that is used when not inheriting from a workspace."] Value (TomlDependency) , # [doc = " The type when inheriting from a workspace."] Inherit (TomlInheritedDependency) , }
    };
}

InheritableDependency!()