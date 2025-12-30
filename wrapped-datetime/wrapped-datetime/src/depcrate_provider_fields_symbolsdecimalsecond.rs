// Generated macro for DecimalSecond (enum)
macro_rules! Depcrate_provider_fields_symbolsDecimalSecond {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"DecimalSecond"}
// Dependencies: {}
# [doc = " A second field with fractional digits."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Clone , Copy , yoke :: Yokeable , zerofrom :: ZeroFrom ,)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: fields))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [repr (u8)] # [zerovec :: make_ule (DecimalSecondULE)] # [zerovec :: derive (Debug)] # [allow (clippy :: exhaustive_enums)] pub enum DecimalSecond { # [doc = " A second with 1 fractional digit: \"1.0\""] Subsecond1 = 1 , # [doc = " A second with 2 fractional digits: \"1.00\""] Subsecond2 = 2 , # [doc = " A second with 3 fractional digits: \"1.000\""] Subsecond3 = 3 , # [doc = " A second with 4 fractional digits: \"1.0000\""] Subsecond4 = 4 , # [doc = " A second with 5 fractional digits: \"1.00000\""] Subsecond5 = 5 , # [doc = " A second with 6 fractional digits: \"1.000000\""] Subsecond6 = 6 , # [doc = " A second with 7 fractional digits: \"1.0000000\""] Subsecond7 = 7 , # [doc = " A second with 8 fractional digits: \"1.00000000\""] Subsecond8 = 8 , # [doc = " A second with 9 fractional digits: \"1.000000000\""] Subsecond9 = 9 , }
};
}
