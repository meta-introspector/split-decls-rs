// Generated macro for Base (enum)
macro_rules! Depcrate_measure_provider_si_prefixBase {
() => {
// Module: crate::measure::provider::si_prefix
// Provides: {"Base"}
// Dependencies: {}
# [doc = " Represents the base of an si prefix."] # [zerovec :: make_ule (BaseULE)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: measure :: provider :: si_prefix))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq , Default)] # [repr (u8)] pub enum Base { # [doc = " The base of the si prefix is 10."] # [default] Decimal = 0 , # [doc = " The base of the si prefix is 2."] Binary = 1 , }
};
}
