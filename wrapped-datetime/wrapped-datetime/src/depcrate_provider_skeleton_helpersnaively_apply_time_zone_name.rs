// Generated macro for naively_apply_time_zone_name (function)
macro_rules! Depcrate_provider_skeleton_helpersnaively_apply_time_zone_name {
() => {
// Module: crate::provider::skeleton::helpers
// Provides: {"naively_apply_time_zone_name"}
// Dependencies: {}
# [doc = " This function swaps out the time zone name field for the appropriate one. Skeleton matching"] # [doc = " only needs to find a single \"v\" field, and then the time zone name can expand from there."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] fn naively_apply_time_zone_name (pattern : & mut runtime :: Pattern , time_zone_name : Option < components :: TimeZoneName > ,) { if let Some (time_zone_name) = time_zone_name { runtime :: helpers :: maybe_replace_first (pattern , | item | { if let PatternItem :: Field (fields :: Field { symbol : fields :: FieldSymbol :: TimeZone (_) , length : _ , }) = item { Some (PatternItem :: Field ((time_zone_name) . into ())) } else { None } }) ; } }
};
}
