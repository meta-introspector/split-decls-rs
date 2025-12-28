macro_rules! deps {
    () => {
        Idx!();
        ArenaMapIter!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T , V > Iterator for ArenaMapIter < Idx < T > , V > { type Item = (Idx < T > , V) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { for next in self . iter . by_ref () { match Self :: mapper (next) { Some (r) => return Some (r) , None => continue , } } None } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_10!();