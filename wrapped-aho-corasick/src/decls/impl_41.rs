macro_rules! deps {
    () => {
        Match!();
        Automaton!();
        FindOverlappingIter!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'a , 'h , A : Automaton > Iterator for FindOverlappingIter < 'a , 'h , A > { type Item = Match ; # [inline (always)] fn next (& mut self) -> Option < Match > { self . aut . try_find_overlapping (& self . input , & mut self . state) . expect ("already checked that no match error can occur here") ; self . state . get_match () } }
    };
}

impl_41!()