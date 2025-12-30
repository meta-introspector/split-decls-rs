// Generated macro for LocaleDisplayNames (struct)
macro_rules! Depcrate_displaynames_providerLocaleDisplayNames {
() => {
// Module: crate::displaynames::provider
// Provides: {"LocaleDisplayNames"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , Default , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: displaynames :: provider))] # [yoke (prove_covariance_manually)] # [doc = " LocaleDisplayNames provides mapping between locales and display names."] pub struct LocaleDisplayNames < 'data > { # [doc = " Mapping for locale to display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub names : ZeroMap < 'data , UnvalidatedLocale , str > , # [doc = " Mapping for locale to short display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub short_names : ZeroMap < 'data , UnvalidatedLocale , str > , # [doc = " Mapping for locale to long display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub long_names : ZeroMap < 'data , UnvalidatedLocale , str > , # [doc = " Mapping for locale to menu variant display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub menu_names : ZeroMap < 'data , UnvalidatedLocale , str > , }
};
}
