macro_rules! deps {
    () => {
        FindMatches!();
        Match!();
        Automaton!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < 'r , 'h , A : Automaton > Iterator for FindMatches < 'r , 'h , A > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { let FindMatches { re , ref mut it } = * self ; it . advance (| input | re . try_search (input)) } }
    };
}

impl_110!();