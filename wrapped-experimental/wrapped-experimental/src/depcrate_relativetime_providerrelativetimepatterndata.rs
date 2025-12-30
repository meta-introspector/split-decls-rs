// Generated macro for RelativeTimePatternData (struct)
macro_rules! Depcrate_relativetime_providerRelativeTimePatternData {
() => {
// Module: crate::relativetime::provider
// Provides: {"RelativeTimePatternData"}
// Dependencies: {}
# [doc = " Relative time format  data struct."] # [derive (Debug , Clone , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: relativetime :: provider))] # [yoke (prove_covariance_manually)] pub struct RelativeTimePatternData < 'data > { # [doc = " Mapping for relative times with unique names."] # [doc = " Example."] # [doc = " In English, \"-1\" corresponds to \"yesterday\", \"1\" corresponds to \"tomorrow\"."] # [cfg_attr (feature = "serde" , serde (borrow))] pub relatives : ZeroMap < 'data , i8 , str > , # [doc = " How to display times in the past."] # [cfg_attr (feature = "serde" , serde (borrow))] pub past : PluralElementsPackedCow < 'data , SinglePlaceholderPattern > , # [doc = " How to display times in the future."] # [cfg_attr (feature = "serde" , serde (borrow))] pub future : PluralElementsPackedCow < 'data , SinglePlaceholderPattern > , }
};
}
