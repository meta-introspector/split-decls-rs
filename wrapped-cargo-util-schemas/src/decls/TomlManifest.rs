macro_rules! deps {
    () => {
        TomlPackage!();
        TomlLibTarget!();
        InheritableLints!();
        Hints!();
        TomlProfiles!();
        TomlDependency!();
        TomlBinTarget!();
        TomlExampleTarget!();
        InheritableDependency!();
        TomlTestTarget!();
        TomlWorkspace!();
        TomlPlatform!();
    };
}

macro_rules! TomlManifest {
    () => {
        deps!();
        # [doc = " This type is used to deserialize `Cargo.toml` files."] # [derive (Default , Clone , Debug , Deserialize , Serialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlManifest { pub cargo_features : Option < Vec < String > > , pub package : Option < Box < TomlPackage > > , pub project : Option < Box < TomlPackage > > , pub badges : Option < BTreeMap < String , BTreeMap < String , String > > > , pub features : Option < BTreeMap < FeatureName , Vec < String > > > , pub lib : Option < TomlLibTarget > , pub bin : Option < Vec < TomlBinTarget > > , pub example : Option < Vec < TomlExampleTarget > > , pub test : Option < Vec < TomlTestTarget > > , pub bench : Option < Vec < TomlTestTarget > > , pub dependencies : Option < BTreeMap < PackageName , InheritableDependency > > , pub dev_dependencies : Option < BTreeMap < PackageName , InheritableDependency > > , # [serde (rename = "dev_dependencies")] pub dev_dependencies2 : Option < BTreeMap < PackageName , InheritableDependency > > , pub build_dependencies : Option < BTreeMap < PackageName , InheritableDependency > > , # [serde (rename = "build_dependencies")] pub build_dependencies2 : Option < BTreeMap < PackageName , InheritableDependency > > , pub target : Option < BTreeMap < String , TomlPlatform > > , pub lints : Option < InheritableLints > , pub hints : Option < Hints > , pub workspace : Option < TomlWorkspace > , pub profile : Option < TomlProfiles > , pub patch : Option < BTreeMap < String , BTreeMap < PackageName , TomlDependency > > > , pub replace : Option < BTreeMap < String , TomlDependency > > , # [doc = " Report unused keys (see also nested `_unused_keys`)"] # [doc = " Note: this is populated by the caller, rather than automatically"] # [serde (skip)] pub _unused_keys : BTreeSet < String > , }
    };
}

TomlManifest!()