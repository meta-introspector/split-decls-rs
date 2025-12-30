// Generated macro for UnitsInfo (struct)
macro_rules! Depcrate_units_providerUnitsInfo {
() => {
// Module: crate::units::provider
// Provides: {"UnitsInfo"}
// Dependencies: {}
# [doc = " This type encapsulates all the constant data required for unit conversions."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Clone , PartialEq , Debug , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: units :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct UnitsInfo < 'data > { # [doc = " Contains conversion information sorted by unit_id, including conversion rates and base units."] # [doc = " For instance, the conversion for `foot` is represented as `1 foot = 0.3048 meter`."] # [cfg_attr (feature = "serde" , serde (borrow))] pub conversion_info : VarZeroVec < 'data , ConversionInfoULE > , }
};
}
