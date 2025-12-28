macro_rules! deps {
    () => {
        Match!();
        FindMatches!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < 'r , 'c , 'h > Iterator for FindMatches < 'r , 'c , 'h > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { let FindMatches { re , ref mut cache , ref mut it } = * self ; it . advance (| input | re . try_search (cache , input)) } }
    };
}

impl_279!();