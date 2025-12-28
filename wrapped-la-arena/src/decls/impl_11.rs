macro_rules! deps {
    () => {
        Idx!();
        ArenaMapIter!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T , V > DoubleEndedIterator for ArenaMapIter < Idx < T > , V > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { while let Some (next_back) = self . iter . next_back () { match Self :: mapper (next_back) { Some (r) => return Some (r) , None => continue , } } None } }
    };
}

impl_11!()