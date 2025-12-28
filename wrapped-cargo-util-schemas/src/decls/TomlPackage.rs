macro_rules! deps {
    () => {
        InvalidCargoFeatures!();
        StringOrVec!();
        InheritableString!();
        InheritableStringOrBool!();
        InheritableVecStringOrBool!();
        InheritableRustVersion!();
        InheritableVecString!();
        InheritableSemverVersion!();
        TomlPackageBuild!();
    };
}

macro_rules! TomlPackage {
    () => {
        deps!();
        # [doc = " Represents the `package`/`project` sections of a `Cargo.toml`."] # [doc = ""] # [doc = " Note that the order of the fields matters, since this is the order they"] # [doc = " are serialized to a TOML file. For example, you cannot have values after"] # [doc = " the field `metadata`, since it is a table and values cannot appear after"] # [doc = " tables."] # [derive (Deserialize , Serialize , Clone , Debug , Default)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlPackage { pub edition : Option < InheritableString > , # [cfg_attr (feature = "unstable-schema" , schemars (with = "Option<String>"))] pub rust_version : Option < InheritableRustVersion > , # [cfg_attr (feature = "unstable-schema" , schemars (with = "Option<String>"))] pub name : Option < PackageName > , pub version : Option < InheritableSemverVersion > , pub authors : Option < InheritableVecString > , pub build : Option < TomlPackageBuild > , pub metabuild : Option < StringOrVec > , pub default_target : Option < String > , pub forced_target : Option < String > , pub links : Option < String > , pub exclude : Option < InheritableVecString > , pub include : Option < InheritableVecString > , pub publish : Option < InheritableVecStringOrBool > , pub workspace : Option < String > , pub im_a_teapot : Option < bool > , pub autolib : Option < bool > , pub autobins : Option < bool > , pub autoexamples : Option < bool > , pub autotests : Option < bool > , pub autobenches : Option < bool > , pub default_run : Option < String > , pub description : Option < InheritableString > , pub homepage : Option < InheritableString > , pub documentation : Option < InheritableString > , pub readme : Option < InheritableStringOrBool > , pub keywords : Option < InheritableVecString > , pub categories : Option < InheritableVecString > , pub license : Option < InheritableString > , pub license_file : Option < InheritableString > , pub repository : Option < InheritableString > , pub resolver : Option < String > , # [cfg_attr (feature = "unstable-schema" , schemars (with = "Option<TomlValueWrapper>"))] pub metadata : Option < toml :: Value > , # [doc = " Provide a helpful error message for a common user error."] # [serde (rename = "cargo-features" , skip_serializing)] # [cfg_attr (feature = "unstable-schema" , schemars (skip))] pub _invalid_cargo_features : Option < InvalidCargoFeatures > , }
    };
}

TomlPackage!();