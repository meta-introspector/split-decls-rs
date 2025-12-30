// Generated macro for RegionNameSource (enum)
macro_rules! Depcrate_diagnostics_region_nameRegionNameSource {
() => {
// Module: crate::diagnostics::region_name
// Provides: {"RegionNameSource"}
// Dependencies: {}
# [doc = " Denotes the source of a region that is named by a `RegionName`. For example, a free region that"] # [doc = " was named by the user would get `NamedLateParamRegion` and `'static` lifetime would get"] # [doc = " `Static`. This helps to print the right kinds of diagnostics."] # [derive (Debug , Clone , Copy)] pub (crate) enum RegionNameSource { # [doc = " A bound (not free) region that was instantiated at the def site (not an HRTB)."] NamedEarlyParamRegion (Span) , # [doc = " A free region that the user has a name (`'a`) for."] NamedLateParamRegion (Span) , # [doc = " The `'static` region."] Static , # [doc = " The free region corresponding to the environment of a closure."] SynthesizedFreeEnvRegion (Span , & 'static str) , # [doc = " The region corresponding to an argument."] AnonRegionFromArgument (RegionNameHighlight) , # [doc = " The region corresponding to a closure upvar."] AnonRegionFromUpvar (Span , Symbol) , # [doc = " The region corresponding to the return type of a closure."] AnonRegionFromOutput (RegionNameHighlight , & 'static str) , # [doc = " The region from a type yielded by a coroutine."] AnonRegionFromYieldTy (Span , Symbol) , # [doc = " An anonymous region from an async fn."] AnonRegionFromAsyncFn (Span) , # [doc = " An anonymous region from an impl self type or trait"] AnonRegionFromImplSignature (Span , & 'static str) , }
};
}
