// Generated macro for DigitalDurationData (struct)
macro_rules! Depcrate_duration_providerDigitalDurationData {
() => {
// Module: crate::duration::provider
// Provides: {"DigitalDurationData"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: duration :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [doc = " A struct containing digital duration data (durationUnit-type-* patterns)."] pub struct DigitalDurationData < 'data > { # [doc = " The separator between the hour, minute, and second fields."] # [cfg_attr (feature = "serde" , serde (borrow))] pub separator : Cow < 'data , str > , # [doc = " The number of digits to pad hours when hour, minutes and seconds must be displayed."] # [doc = " Calculated from the hms pattern."] pub hms_padding : HmsPadding , # [doc = " The number of digits to pad hours when only hour and minutes must be displayed."] # [doc = " Calculated from the hm pattern."] pub hm_padding : HmPadding , # [doc = " The number of digits to pad minutes when only minutes and seconds must be displayed."] # [doc = " Calculated from the ms pattern."] pub ms_padding : MsPadding , }
};
}
