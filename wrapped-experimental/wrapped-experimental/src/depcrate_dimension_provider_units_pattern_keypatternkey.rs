// Generated macro for PatternKey (enum)
macro_rules! Depcrate_dimension_provider_units_pattern_keyPatternKey {
() => {
// Module: crate::dimension::provider::units::pattern_key
// Provides: {"PatternKey"}
// Dependencies: {}
# [derive (Copy , Clone , PartialOrd , Ord , PartialEq , Eq , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] pub enum PatternKey { Binary (u8) , Decimal (i8) , Power { power : PowerValue , count : CompoundCount , } , }
};
}
