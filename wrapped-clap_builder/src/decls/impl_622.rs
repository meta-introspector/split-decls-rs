macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < (& 'a K , & 'a V) > { match self . keys . next () { Some (k) => { let v = self . values . next () . unwrap () ; Some ((k , v)) } None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { self . keys . size_hint () } }
    };
}

impl_622!();