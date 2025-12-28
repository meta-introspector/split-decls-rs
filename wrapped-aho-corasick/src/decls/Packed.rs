macro_rules! deps {
    () => {
        Searcher!();
        Prefilter!();
    };
}

macro_rules! Packed {
    () => {
        deps!();
        # [doc = " A type that wraps a packed searcher and implements the `Prefilter`"] # [doc = " interface."] # [derive (Clone , Debug)] struct Packed (packed :: Searcher) ;
    };
}

Packed!();