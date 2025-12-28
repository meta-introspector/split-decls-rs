macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < 'h , 's > Iterator for Split < 'h , 's > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { let haystack = self . finder . haystack ; match self . finder . next () { Some (start) => { let next = & haystack [self . last .. start] ; self . last = start + self . finder . needle . len () ; Some (next) } None => { if self . last >= haystack . len () { if ! self . done { self . done = true ; Some (b"") } else { None } } else { let s = & haystack [self . last ..] ; self . last = haystack . len () ; self . done = true ; Some (s) } } } } }
    };
}

impl_90!()