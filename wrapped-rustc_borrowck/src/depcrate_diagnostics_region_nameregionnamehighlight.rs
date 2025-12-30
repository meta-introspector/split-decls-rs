// Generated macro for RegionNameHighlight (enum)
macro_rules! Depcrate_diagnostics_region_nameRegionNameHighlight {
() => {
// Module: crate::diagnostics::region_name
// Provides: {"RegionNameHighlight"}
// Dependencies: {}
# [doc = " Describes what to highlight to explain to the user that we're giving an anonymous region a"] # [doc = " synthesized name, and how to highlight it."] # [derive (Debug , Clone , Copy)] pub (crate) enum RegionNameHighlight { # [doc = " The anonymous region corresponds to a reference that was found by traversing the type in the HIR."] MatchedHirTy (Span) , # [doc = " The anonymous region corresponds to a `'_` in the generics list of a struct/enum/union."] MatchedAdtAndSegment (Span) , # [doc = " The anonymous region corresponds to a region where the type annotation is completely missing"] # [doc = " from the code, e.g. in a closure arguments `|x| { ... }`, where `x` is a reference."] CannotMatchHirTy (Span , Symbol) , # [doc = " The anonymous region corresponds to a region where the type annotation is completely missing"] # [doc = " from the code, and *even if* we print out the full name of the type, the region name won't"] # [doc = " be included. This currently occurs for opaque types like `impl Future`."] Occluded (Span , Symbol) , }
};
}
