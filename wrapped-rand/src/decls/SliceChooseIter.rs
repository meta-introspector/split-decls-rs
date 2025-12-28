macro_rules! deps {
    () => {
        IndexedSamples!();
    };
}

macro_rules! SliceChooseIter {
    () => {
        deps!();
        # [doc = " Deprecated: renamed to [`IndexedSamples`]"] # [cfg (feature = "alloc")] # [deprecated (since = "0.9.2" , note = "Renamed to `IndexedSamples`")] pub type SliceChooseIter < 'a , S , T > = IndexedSamples < 'a , S , T > ;
    };
}

SliceChooseIter!();