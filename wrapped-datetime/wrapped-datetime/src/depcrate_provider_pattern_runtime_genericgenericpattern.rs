// Generated macro for GenericPattern (struct)
macro_rules! Depcrate_provider_pattern_runtime_genericGenericPattern {
() => {
// Module: crate::provider::pattern::runtime::generic
// Provides: {"GenericPattern"}
// Dependencies: {}
# [doc = " A raw, low-level pattern with literals and placeholders."] # [doc = ""] # [doc = " This is a datetime-specific type designed to be binary-compatible with"] # [doc = " [`Pattern`]. ICU4X developers looking for this sort of type should use"] # [doc = " the `icu_pattern` crate instead."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Eq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [allow (clippy :: exhaustive_structs)] # [cfg_attr (feature = "datagen" , derive (databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: provider :: pattern :: runtime))] pub struct GenericPattern < 'data > { # [doc = " The list of [`GenericPatternItem`]s."] pub items : ZeroVec < 'data , GenericPatternItem > , }
};
}
