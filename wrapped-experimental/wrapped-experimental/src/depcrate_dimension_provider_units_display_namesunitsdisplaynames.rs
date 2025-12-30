// Generated macro for UnitsDisplayNames (struct)
macro_rules! Depcrate_dimension_provider_units_display_namesUnitsDisplayNames {
() => {
// Module: crate::dimension::provider::units::display_names
// Provides: {"UnitsDisplayNames"}
// Dependencies: {}
# [derive (Clone , PartialEq , Debug , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [yoke (prove_covariance_manually)] pub struct UnitsDisplayNames < 'data > { # [doc = " Contains the long width patterns for the units."] # [cfg_attr (feature = "serde" , serde (borrow))] pub patterns : PluralElementsPackedCow < 'data , SinglePlaceholderPattern > , }
};
}
