// Generated macro for LexicalRegionResolutions (struct)
macro_rules! Depcrate_infer_lexical_region_resolveLexicalRegionResolutions {
() => {
// Module: crate::infer::lexical_region_resolve
// Provides: {"LexicalRegionResolutions"}
// Dependencies: {}
# [doc = " Contains the result of lexical region resolution. Offers methods"] # [doc = " to lookup up the final value of a region variable."] # [derive (Clone)] pub (crate) struct LexicalRegionResolutions < 'tcx > { pub (crate) values : IndexVec < RegionVid , VarValue < 'tcx > > , }
};
}
