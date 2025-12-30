// Generated macro for ScriptDisplayNames (struct)
macro_rules! Depcrate_displaynames_providerScriptDisplayNames {
() => {
// Module: crate::displaynames::provider
// Provides: {"ScriptDisplayNames"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , Default , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: displaynames :: provider))] # [yoke (prove_covariance_manually)] # [doc = " ScriptDisplayNames provides mapping between a script code and it's display name."] pub struct ScriptDisplayNames < 'data > { # [doc = " Mapping for script to locale display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub names : ZeroMap < 'data , UnvalidatedScript , str > , # [doc = " Mapping for script to locale display short name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub short_names : ZeroMap < 'data , UnvalidatedScript , str > , }
};
}
