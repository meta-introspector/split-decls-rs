// Generated macro for TomlPlatform (struct)
macro_rules! Depcrate_manifestTomlPlatform {
() => {
// Module: crate::manifest
// Provides: {"TomlPlatform"}
// Dependencies: {}
# [doc = " Corresponds to a `target` entry, but `TomlTarget` is already used."] # [derive (Serialize , Deserialize , Debug , Clone)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlPlatform { pub dependencies : Option < BTreeMap < PackageName , InheritableDependency > > , pub build_dependencies : Option < BTreeMap < PackageName , InheritableDependency > > , # [serde (rename = "build_dependencies")] pub build_dependencies2 : Option < BTreeMap < PackageName , InheritableDependency > > , pub dev_dependencies : Option < BTreeMap < PackageName , InheritableDependency > > , # [serde (rename = "dev_dependencies")] pub dev_dependencies2 : Option < BTreeMap < PackageName , InheritableDependency > > , }
};
}
