// Generated macro for RegionRelations (struct)
macro_rules! Depcrate_infer_free_regionsRegionRelations {
() => {
// Module: crate::infer::free_regions
// Provides: {"RegionRelations"}
// Dependencies: {}
# [doc = " Combines a `FreeRegionMap` and a `TyCtxt`."] # [doc = ""] # [doc = " This stuff is a bit convoluted and should be refactored, but as we"] # [doc = " transition to NLL, it'll all go away anyhow."] pub (crate) struct RegionRelations < 'a , 'tcx > { pub tcx : TyCtxt < 'tcx > , # [doc = " Free-region relationships."] pub free_regions : & 'a FreeRegionMap < 'tcx > , }
};
}
