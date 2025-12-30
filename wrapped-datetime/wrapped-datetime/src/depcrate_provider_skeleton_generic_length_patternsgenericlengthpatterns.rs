// Generated macro for GenericLengthPatterns (struct)
macro_rules! Depcrate_provider_skeleton_generic_length_patternsGenericLengthPatterns {
() => {
// Module: crate::provider::skeleton::generic_length_patterns
// Provides: {"GenericLengthPatterns"}
// Dependencies: {}
# [doc = " A struct containing `dateTimePatterns` aka \"glue patterns\"."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , Default , yoke :: Yokeable , zerofrom :: ZeroFrom)] pub struct GenericLengthPatterns < 'data > { # [doc = " A full length glue pattern of other formatted elements."] pub full : runtime :: GenericPattern < 'data > , # [doc = " A long length glue pattern of other formatted elements."] pub long : runtime :: GenericPattern < 'data > , # [doc = " A medium length glue pattern of other formatted elements."] pub medium : runtime :: GenericPattern < 'data > , # [doc = " A short length glue pattern of other formatted elements."] pub short : runtime :: GenericPattern < 'data > , }
};
}
