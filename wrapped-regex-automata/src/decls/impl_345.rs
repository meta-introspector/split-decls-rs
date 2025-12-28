macro_rules! deps {
    () => {
        FindMatches!();
        Match!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < 'r , 'h > Iterator for FindMatches < 'r , 'h > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { let FindMatches { re , ref mut cache , ref mut it } = * self ; it . advance (| input | Ok (re . search_with (cache , input))) } # [inline] fn count (self) -> usize { let FindMatches { re , mut cache , it } = self ; let cache = & mut * cache ; it . into_half_matches_iter (| input | Ok (re . search_half_with (cache , input)) ,) . count () } }
    };
}

impl_345!()