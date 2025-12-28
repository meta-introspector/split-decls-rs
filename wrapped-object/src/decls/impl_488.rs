macro_rules! deps {
    () => {
        ReadRef!();
        DyldRelocation!();
        Endian!();
        Item!();
        DyldCacheRelocationIterator!();
        DyldCacheRelocationIteratorVersion!();
        Result!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl < 'data , E , R > Iterator for DyldCacheRelocationIterator < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { type Item = Result < DyldRelocation > ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . version { DyldCacheRelocationIteratorVersion :: None => Ok (None) , DyldCacheRelocationIteratorVersion :: V2 (iter) => iter . next () , DyldCacheRelocationIteratorVersion :: V3 (iter) => iter . next () , DyldCacheRelocationIteratorVersion :: V5 (iter) => iter . next () , } . transpose () } }
    };
}

impl_488!();