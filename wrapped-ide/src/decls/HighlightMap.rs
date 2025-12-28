macro_rules! deps {
    () => {
        HighlightedRange!();
    };
}

macro_rules! HighlightMap {
    () => {
        deps!();
        type HighlightMap = FxHashMap < EditionedFileId , FxHashSet < HighlightedRange > > ;
    };
}

HighlightMap!()