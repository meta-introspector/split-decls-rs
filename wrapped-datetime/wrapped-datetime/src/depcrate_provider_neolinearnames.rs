// Generated macro for LinearNames (struct)
macro_rules! Depcrate_provider_neoLinearNames {
() => {
// Module: crate::provider::neo
// Provides: {"LinearNames"}
// Dependencies: {}
# [doc = " Names that can be stored as a simple linear array."] # [doc = ""] # [doc = " - For weekdays, element 0 is Sunday"] # [doc = " - For dayperiods, the elements are in order: AM, PM, (noon), (midnight), where the latter two are optional."] # [doc = "   In the case noon is missing but midnight is present, the noon value can be the empty string. This is unlikely."] # [doc = " - For day names element 0 is the first day of the month"] # [doc = ""] # [doc = " This uses a data marker attribute for length. See [`YearNames`] for more information on the scheme."] # [doc = linear_names_v1_size ! ()] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: provider :: neo))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [yoke (prove_covariance_manually)] pub struct LinearNames < 'data > { # [cfg_attr (feature = "serde" , serde (borrow))] # [doc = " The names, in order. Order specified on the struct docs."] pub names : VarZeroVec < 'data , str > , }
};
}
