macro_rules! deps {
    () => {
        FreeRegionMap!();
    };
}

macro_rules! RegionRelations {
    () => {
        deps!();
        # [doc = " Combines a `FreeRegionMap` and a `TyCtxt`."] # [doc = ""] # [doc = " This stuff is a bit convoluted and should be refactored, but as we"] # [doc = " transition to NLL, it'll all go away anyhow."] pub (crate) struct RegionRelations < 'a , 'tcx > { pub tcx : TyCtxt < 'tcx > , # [doc = " Free-region relationships."] pub free_regions : & 'a FreeRegionMap < 'tcx > , }
    };
}

RegionRelations!()