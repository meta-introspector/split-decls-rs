// Generated macro for Parents (struct)
macro_rules! Depcrate_providerParents {
() => {
// Module: crate::provider
// Provides: {"Parents"}
// Dependencies: {}
# [doc = " Locale fallback rules derived from CLDR parent locales data."] # [derive (Default , Clone , PartialEq , Debug , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_locale :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [yoke (prove_covariance_manually)] pub struct Parents < 'data > { # [doc = " Map from language identifier to language identifier, indicating that the language on the"] # [doc = " left should inherit from the language on the right."] # [cfg_attr (feature = "serde" , serde (borrow))] pub parents : ZeroMap < 'data , PotentialUtf8 , (Language , Option < Script > , Option < Region >) > , }
};
}
