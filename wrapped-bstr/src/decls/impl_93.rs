macro_rules! deps {
    () => {
        SplitReverse!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'h , 's > Iterator for SplitReverse < 'h , 's > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { let haystack = self . finder . haystack () ; match self . finder . next () { Some (start) => { let nlen = self . finder . needle () . len () ; let next = & haystack [start + nlen .. self . last] ; self . last = start ; Some (next) } None => { if self . last == 0 { if ! self . done { self . done = true ; Some (b"") } else { None } } else { let s = & haystack [.. self . last] ; self . last = 0 ; self . done = true ; Some (s) } } } } }
    };
}

impl_93!();