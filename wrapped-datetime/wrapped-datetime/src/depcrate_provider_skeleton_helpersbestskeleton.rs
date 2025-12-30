// Generated macro for BestSkeleton (enum)
macro_rules! Depcrate_provider_skeleton_helpersBestSkeleton {
() => {
// Module: crate::provider::skeleton::helpers
// Provides: {"BestSkeleton"}
// Dependencies: {}
# [doc = " The best skeleton found, alongside information on how well it matches."] # [doc = ""] # [doc = " According to the [UTS 35 skeleton matching algorithm](https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons)"] # [doc = " there will be a guaranteed match for a skeleton. However, with this initial implementation,"] # [doc = " there is no attempt to add on missing fields. This enum encodes the variants for the current"] # [doc = " search for a best skeleton."] # [doc = ""] # [doc = " The patterns are paired with a measure of their quality."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone)] # [allow (missing_docs)] pub enum BestSkeleton < T > { AllFieldsMatch (T , SkeletonQuality) , MissingOrExtraFields (T , SkeletonQuality) , NoMatch , }
};
}
