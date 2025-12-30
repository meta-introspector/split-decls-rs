// Generated macro for Pattern (struct)
macro_rules! Depcrate_provider_pattern_runtime_patternPattern {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"Pattern"}
// Dependencies: {}
# [doc = " A raw, low-level pattern for datetime formatting."] # [doc = ""] # [doc = " It consists of an owned-or-borrowed list of [`PatternItem`]s corresponding"] # [doc = " to either fields or literal characters."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Eq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: provider :: pattern :: runtime))] # [zerovec :: make_varule (PatternULE)] # [zerovec :: derive (Debug)] # [zerovec :: skip_derive (Ord)] # [cfg_attr (feature = "serde" , zerovec :: derive (Deserialize))] # [cfg_attr (feature = "datagen" , zerovec :: derive (Serialize))] pub struct Pattern < 'data > { # [doc = " The list of [`PatternItem`]s."] pub items : ZeroVec < 'data , PatternItem > , # [doc = " Pre-computed metadata about the pattern."] # [doc = ""] # [doc = " This field should contain the smallest time unit from the `items` vec."] # [doc = " If it doesn't, unexpected results for day periods may be encountered."] pub metadata : PatternMetadata , }
};
}
