macro_rules! deps {
    () => {
        FileReference!();
    };
}

macro_rules! UsageSearchResult {
    () => {
        deps!();
        # [derive (Debug , Default , Clone)] pub struct UsageSearchResult { pub references : FxHashMap < EditionedFileId , Vec < FileReference > > , }
    };
}

UsageSearchResult!()