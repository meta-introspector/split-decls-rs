macro_rules! deps {
    () => {
        TomlDetailedDependency!();
    };
}

macro_rules! TomlDependency {
    () => {
        deps!();
        # [derive (Clone , Debug , Serialize)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlDependency < P : Clone = String > { # [doc = " In the simple format, only a version is specified, eg."] # [doc = " `package = \"<version>\"`"] Simple (String) , # [doc = " The simple format is equivalent to a detailed dependency"] # [doc = " specifying only a version, eg."] # [doc = " `package = { version = \"<version>\" }`"] Detailed (TomlDetailedDependency < P >) , }
    };
}

TomlDependency!()