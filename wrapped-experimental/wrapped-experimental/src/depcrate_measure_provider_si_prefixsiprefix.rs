// Generated macro for SiPrefix (struct)
macro_rules! Depcrate_measure_provider_si_prefixSiPrefix {
() => {
// Module: crate::measure::provider::si_prefix
// Provides: {"SiPrefix"}
// Dependencies: {}
# [doc = " Represents the SI prefix."] # [zerovec :: make_ule (SiPrefixULE)] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq , Default)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: measure :: provider :: si_prefix))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct SiPrefix { # [doc = " The absolute value of the power of the si prefix."] pub power : i8 , # [doc = " The base of the si prefix."] pub base : Base , }
};
}
