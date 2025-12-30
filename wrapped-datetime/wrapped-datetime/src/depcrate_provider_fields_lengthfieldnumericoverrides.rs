// Generated macro for FieldNumericOverrides (enum)
macro_rules! Depcrate_provider_fields_lengthFieldNumericOverrides {
() => {
// Module: crate::provider::fields::length
// Provides: {"FieldNumericOverrides"}
// Dependencies: {}
# [doc = " Various numeric overrides for datetime patterns"] # [doc = " as found in CLDR"] # [derive (Debug , Eq , PartialEq , Clone , Copy , Ord , PartialOrd)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: fields))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [non_exhaustive] pub enum FieldNumericOverrides { # [doc = " `hanidec`"] Hanidec = 0 , # [doc = " `hanidays`"] Hanidays = 1 , # [doc = " `hebr`"] Hebr = 2 , # [doc = " `romanlow`"] Romanlow = 3 , # [doc = " `jpnyear`"] Jpnyear = 4 , }
};
}
