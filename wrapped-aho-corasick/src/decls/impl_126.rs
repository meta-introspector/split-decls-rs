macro_rules! deps {
    () => {
        PatternID!();
        Pattern!();
        PatternIter!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'p > Iterator for PatternIter < 'p > { type Item = (PatternID , Pattern < 'p >) ; fn next (& mut self) -> Option < (PatternID , Pattern < 'p >) > { if self . i >= self . patterns . len () { return None ; } let id = self . patterns . order [self . i] ; let p = self . patterns . get (id) ; self . i += 1 ; Some ((id , p)) } }
    };
}

impl_126!()