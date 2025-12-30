// Generated macro for TomlProfiles (struct)
macro_rules! Depcrate_manifestTomlProfiles {
() => {
// Module: crate::manifest
// Provides: {"TomlProfiles"}
// Dependencies: {}
# [derive (Deserialize , Serialize , Clone , Debug , Default)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct TomlProfiles (pub BTreeMap < ProfileName , TomlProfile >) ;
};
}
