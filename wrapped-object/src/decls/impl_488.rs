macro_rules! deps {
    () => {
        ReadRef!();
        DyldCacheRelocationIterator!();
        DyldRelocation!();
        DyldCacheRelocationIteratorVersion!();
        Endian!();
        Result!();
        Item!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl < 'data , E , R > Iterator for DyldCacheRelocationIterator < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { type Item = Result < DyldRelocation > ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . version { DyldCacheRelocationIteratorVersion :: None => Ok (None) , DyldCacheRelocationIteratorVersion :: V2 (iter) => iter . next () , DyldCacheRelocationIteratorVersion :: V3 (iter) => iter . next () , DyldCacheRelocationIteratorVersion :: V5 (iter) => iter . next () , } . transpose () } }
    };
}

impl_488!()