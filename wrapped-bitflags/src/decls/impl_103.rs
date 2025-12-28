macro_rules! deps {
    () => {
        Flags!();
        Iter!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < B : Flags > Iterator for Iter < B > { type Item = B ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some ((_ , flag)) => Some (flag) , None if ! self . done => { self . done = true ; if ! self . inner . remaining () . is_empty () { Some (B :: from_bits_retain (self . inner . remaining . bits ())) } else { None } } None => None , } } }
    };
}

impl_103!()