macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_627 {
    () => {
        deps!();
        impl < 'a , K , V > DoubleEndedIterator for IterMut < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a mut V) > { match self . keys . next_back () { Some (k) => { let v = self . values . next_back () . unwrap () ; Some ((k , v)) } None => None , } } }
    };
}

impl_627!()