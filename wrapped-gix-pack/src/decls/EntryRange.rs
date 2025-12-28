macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! EntryRange {
    () => {
        deps!();
        # [doc = " A slice into a pack file denoting a pack entry."] # [doc = ""] # [doc = " An entry can be decoded into an object."] pub type EntryRange = std :: ops :: Range < Offset > ;
    };
}

EntryRange!();