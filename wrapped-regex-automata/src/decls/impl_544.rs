macro_rules! deps {
    () => {
        FindMatches!();
        Match!();
    };
}

macro_rules! impl_544 {
    () => {
        deps!();
        impl < 'r , 'c , 'h > Iterator for FindMatches < 'r , 'c , 'h > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { let FindMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; it . advance (| input | { re . search (cache , input , caps) ; Ok (caps . get_match ()) }) } }
    };
}

impl_544!();