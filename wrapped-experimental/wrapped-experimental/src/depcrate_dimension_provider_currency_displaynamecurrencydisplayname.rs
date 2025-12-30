// Generated macro for CurrencyDisplayname (struct)
macro_rules! Depcrate_dimension_provider_currency_displaynameCurrencyDisplayname {
() => {
// Module: crate::dimension::provider::currency::displayname
// Provides: {"CurrencyDisplayname"}
// Dependencies: {}
# [doc = " Currency Extended  data struct."] # [derive (Debug , Clone , Default , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: currency :: displayname))] # [yoke (prove_covariance_manually)] pub struct CurrencyDisplayname < 'data > { # [doc = " The display name for the currency."] # [cfg_attr (feature = "serde" , serde (borrow))] pub display_name : Cow < 'data , str > , }
};
}
