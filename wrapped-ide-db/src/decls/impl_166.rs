macro_rules! deps {
    () => {
        SearchScope!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl IntoIterator for SearchScope { type Item = (EditionedFileId , Option < TextRange >) ; type IntoIter = std :: collections :: hash_map :: IntoIter < EditionedFileId , Option < TextRange > > ; fn into_iter (self) -> Self :: IntoIter { self . entries . into_iter () } }
    };
}

impl_166!()