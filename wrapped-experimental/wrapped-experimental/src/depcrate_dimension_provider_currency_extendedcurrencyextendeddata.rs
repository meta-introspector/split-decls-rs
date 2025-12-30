// Generated macro for CurrencyExtendedData (struct)
macro_rules! Depcrate_dimension_provider_currency_extendedCurrencyExtendedData {
() => {
// Module: crate::dimension::provider::currency::extended
// Provides: {"CurrencyExtendedData"}
// Dependencies: {}
# [doc = " Currency Extended  data struct."] # [derive (Debug , Clone , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] # [yoke (prove_covariance_manually)] pub struct CurrencyExtendedData < 'data > { # [doc = " Contains the localized display names for a currency based on plural rules."] # [doc = " For instance, in the \"en\" locale for the \"USD\" currency:"] # [doc = "     - \"US Dollars\" when count is `zero`,"] # [doc = "     - \"US Dollar\" when count is `one`,"] # [doc = "     ... etc."] # [doc = " # NOTE"] # [doc = "    Regards to the [Unicode Report TR35](https://unicode.org/reports/tr35/tr35-numbers.html#Currencies),"] # [doc = "    If no matching for specific count, the `other` count will be used."] # [cfg_attr (feature = "serde" , serde (borrow))] pub display_names : PluralElementsPackedCow < 'data , str > , }
};
}
