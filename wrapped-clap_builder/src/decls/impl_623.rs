macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_623 {
    () => {
        deps!();
        impl < 'a , K , V > DoubleEndedIterator for Iter < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a V) > { match self . keys . next_back () { Some (k) => { let v = self . values . next_back () . unwrap () ; Some ((k , v)) } None => None , } } }
    };
}

impl_623!();