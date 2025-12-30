// Generated macro for UniversalRegionIndices (struct)
macro_rules! Depcrate_universal_regionsUniversalRegionIndices {
() => {
// Module: crate::universal_regions
// Provides: {"UniversalRegionIndices"}
// Dependencies: {}
# [derive (Debug)] # [derive (Clone)] struct UniversalRegionIndices < 'tcx > { # [doc = " For those regions that may appear in the parameter environment"] # [doc = " ('static and early-bound regions), we maintain a map from the"] # [doc = " `ty::Region` to the internal `RegionVid` we are using. This is"] # [doc = " used because trait matching and type-checking will feed us"] # [doc = " region constraints that reference those regions and we need to"] # [doc = " be able to map them to our internal `RegionVid`. This is"] # [doc = " basically equivalent to an `GenericArgs`, except that it also"] # [doc = " contains an entry for `ReStatic` -- it might be nice to just"] # [doc = " use an args, and then handle `ReStatic` another way."] indices : FxIndexMap < ty :: Region < 'tcx > , RegionVid > , # [doc = " The vid assigned to `'static`. Used only for diagnostics."] pub fr_static : RegionVid , # [doc = " Whether we've encountered an error region. If we have, cancel all"] # [doc = " outlives errors, as they are likely bogus."] pub encountered_re_error : Cell < Option < ErrorGuaranteed > > , }
};
}
