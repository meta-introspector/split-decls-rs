// Generated macro for InheritablePackage (struct)
macro_rules! Depcrate_manifestInheritablePackage {
() => {
// Module: crate::manifest
// Provides: {"InheritablePackage"}
// Dependencies: {}
# [doc = " A group of fields that are inheritable by members of the workspace"] # [derive (Clone , Debug , Default , Deserialize , Serialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct InheritablePackage { pub version : Option < semver :: Version > , pub authors : Option < Vec < String > > , pub description : Option < String > , pub homepage : Option < String > , pub documentation : Option < String > , pub readme : Option < StringOrBool > , pub keywords : Option < Vec < String > > , pub categories : Option < Vec < String > > , pub license : Option < String > , pub license_file : Option < String > , pub repository : Option < String > , pub publish : Option < VecStringOrBool > , pub edition : Option < String > , pub badges : Option < BTreeMap < String , BTreeMap < String , String > > > , pub exclude : Option < Vec < String > > , pub include : Option < Vec < String > > , # [cfg_attr (feature = "unstable-schema" , schemars (with = "Option<String>"))] pub rust_version : Option < RustVersion > , }
};
}
