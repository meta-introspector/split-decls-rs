// Generated macro for RegionDisplayNames (struct)
macro_rules! Depcrate_displaynames_providerRegionDisplayNames {
() => {
// Module: crate::displaynames::provider
// Provides: {"RegionDisplayNames"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , Default , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: displaynames :: provider))] # [yoke (prove_covariance_manually)] # [doc = " RegionDisplayNames provides mapping between a region code and locale display name."] pub struct RegionDisplayNames < 'data > { # [doc = " Mapping for region to locale display name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub names : ZeroMap < 'data , UnvalidatedRegion , str > , # [doc = " Mapping for region to locale display short name."] # [cfg_attr (feature = "serde" , serde (borrow))] pub short_names : ZeroMap < 'data , UnvalidatedRegion , str > , }
};
}
