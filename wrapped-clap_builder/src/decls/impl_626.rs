macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { match self . keys . next () { Some (k) => { let v = self . values . next () . unwrap () ; Some ((k , v)) } None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { self . keys . size_hint () } }
    };
}

impl_626!();