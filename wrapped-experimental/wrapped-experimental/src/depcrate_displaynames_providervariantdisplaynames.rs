// Generated macro for VariantDisplayNames (struct)
macro_rules! Depcrate_displaynames_providerVariantDisplayNames {
() => {
// Module: crate::displaynames::provider
// Provides: {"VariantDisplayNames"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , Default , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: displaynames :: provider))] # [yoke (prove_covariance_manually)] # [doc = " VariantDisplayNames provides the user-translated names for the variant-code values."] pub struct VariantDisplayNames < 'data > { # [doc = " Mapping for Variant to locale display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub names : ZeroMap < 'data , UnvalidatedVariant , str > , }
};
}
