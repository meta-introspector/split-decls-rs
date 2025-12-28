macro_rules! deps {
    () => {
        TomlLints!();
        TomlDependency!();
        InheritablePackage!();
    };
}

macro_rules! TomlWorkspace {
    () => {
        deps!();
        # [derive (Debug , Default , Deserialize , Serialize , Clone)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlWorkspace { pub members : Option < Vec < String > > , pub exclude : Option < Vec < String > > , pub default_members : Option < Vec < String > > , pub resolver : Option < String > , # [cfg_attr (feature = "unstable-schema" , schemars (with = "Option<TomlValueWrapper>"))] pub metadata : Option < toml :: Value > , pub package : Option < InheritablePackage > , pub dependencies : Option < BTreeMap < PackageName , TomlDependency > > , pub lints : Option < TomlLints > , }
    };
}

TomlWorkspace!();