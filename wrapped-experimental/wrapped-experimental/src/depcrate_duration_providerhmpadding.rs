// Generated macro for HmPadding (struct)
macro_rules! Depcrate_duration_providerHmPadding {
() => {
// Module: crate::duration::provider
// Provides: {"HmPadding"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: duration :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [doc = " A struct containing the number of digits to pad hours and minutes."] pub struct HmPadding { # [doc = " Hour padding."] pub h : u8 , # [doc = " Minute padding."] pub m : u8 , }
};
}
