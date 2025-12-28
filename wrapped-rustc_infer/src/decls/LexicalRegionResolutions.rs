macro_rules! deps {
    () => {
        VarValue!();
    };
}

macro_rules! LexicalRegionResolutions {
    () => {
        deps!();
        # [doc = " Contains the result of lexical region resolution. Offers methods"] # [doc = " to lookup up the final value of a region variable."] # [derive (Clone)] pub (crate) struct LexicalRegionResolutions < 'tcx > { pub (crate) values : IndexVec < RegionVid , VarValue < 'tcx > > , }
    };
}

LexicalRegionResolutions!();