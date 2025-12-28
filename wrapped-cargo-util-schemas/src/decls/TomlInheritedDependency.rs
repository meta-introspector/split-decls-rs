macro_rules! TomlInheritedDependency {
    () => {
        # [derive (Deserialize , Serialize , Clone , Debug)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlInheritedDependency { pub workspace : bool , pub features : Option < Vec < String > > , pub default_features : Option < bool > , # [serde (rename = "default_features")] pub default_features2 : Option < bool > , pub optional : Option < bool > , pub public : Option < bool > , # [doc = " This is here to provide a way to see the \"unused manifest keys\" when deserializing"] # [serde (skip_serializing)] # [serde (flatten)] # [cfg_attr (feature = "unstable-schema" , schemars (skip))] pub _unused_keys : BTreeMap < String , toml :: Value > , }
    };
}

TomlInheritedDependency!();