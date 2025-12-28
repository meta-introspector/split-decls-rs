macro_rules! deps {
    () => {
        FindOverlappingIter!();
        Match!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a , 'h > Iterator for FindOverlappingIter < 'a , 'h > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { self . 0 . next () } }
    };
}

impl_16!()