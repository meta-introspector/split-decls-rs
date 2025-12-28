macro_rules! deps {
    () => {
        FileReference!();
        UsageSearchResult!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl IntoIterator for UsageSearchResult { type Item = (EditionedFileId , Vec < FileReference >) ; type IntoIter = < FxHashMap < EditionedFileId , Vec < FileReference > > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . references . into_iter () } }
    };
}

impl_159!()