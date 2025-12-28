macro_rules! deps {
    () => {
        FindIter!();
        Match!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a , 'h > Iterator for FindIter < 'a , 'h > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { self . 0 . next () } }
    };
}

impl_14!()