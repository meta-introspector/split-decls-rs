// Generated macro for LanguageDisplayNames (struct)
macro_rules! Depcrate_displaynames_providerLanguageDisplayNames {
() => {
// Module: crate::displaynames::provider
// Provides: {"LanguageDisplayNames"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , Default , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: displaynames :: provider))] # [yoke (prove_covariance_manually)] # [doc = " LanguageDisplayNames provides mapping between languages and display names."] pub struct LanguageDisplayNames < 'data > { # [doc = " Mapping for language to display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub names : ZeroMap < 'data , UnvalidatedLanguage , str > , # [doc = " Mapping for language to short display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub short_names : ZeroMap < 'data , UnvalidatedLanguage , str > , # [doc = " Mapping for language to long display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub long_names : ZeroMap < 'data , UnvalidatedLanguage , str > , # [doc = " Mapping for language to menu variant display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub menu_names : ZeroMap < 'data , UnvalidatedLanguage , str > , }
};
}
