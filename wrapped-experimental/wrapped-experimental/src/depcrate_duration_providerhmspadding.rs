// Generated macro for HmsPadding (struct)
macro_rules! Depcrate_duration_providerHmsPadding {
() => {
// Module: crate::duration::provider
// Provides: {"HmsPadding"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: duration :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [doc = " A struct containing the number of digits to pad hours, minutes, and seconds."] pub struct HmsPadding { # [doc = " Hour padding."] pub h : u8 , # [doc = " Minute padding."] pub m : u8 , # [doc = " Second padding."] pub s : u8 , }
};
}
