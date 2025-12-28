macro_rules! deps {
    () => {
        SplitN!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'h , 's > Iterator for SplitN < 'h , 's > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { self . count += 1 ; if self . count > self . limit || self . split . done { None } else if self . count == self . limit { Some (& self . split . finder . haystack [self . split . last ..]) } else { self . split . next () } } }
    };
}

impl_96!()