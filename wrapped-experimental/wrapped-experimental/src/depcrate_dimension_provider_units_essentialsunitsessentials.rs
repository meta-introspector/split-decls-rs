// Generated macro for UnitsEssentials (struct)
macro_rules! Depcrate_dimension_provider_units_essentialsUnitsEssentials {
() => {
// Module: crate::dimension::provider::units::essentials
// Provides: {"UnitsEssentials"}
// Dependencies: {}
# [doc = " This type contains all of the essential data for units formatting such as `per`, `power`, `times`, etc."] # [doc = ""] # [doc = " Note:"] # [doc = "     Auxiliary key represent the length: e.g. `long`, `short`, `narrow`."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Clone , PartialEq , Debug , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: units :: essentials))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [yoke (prove_covariance_manually)] pub struct UnitsEssentials < 'data > { # [cfg_attr (feature = "serde" , serde (borrow))] pub prefixes : ZeroMap < 'data , PatternKey , str > , # [cfg_attr (feature = "serde" , serde (borrow))] pub per : Cow < 'data , str > , # [cfg_attr (feature = "serde" , serde (borrow))] pub times : Cow < 'data , str > , }
};
}
