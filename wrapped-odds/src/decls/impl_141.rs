macro_rules! deps {
    () => {
        CharWindows!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'a > Iterator for CharWindows < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { let elt ; if let Some (c) = char_get (self . s , self . a) { elt = & self . s [self . a .. self . b] ; self . a += c . len_utf8 () ; } else { return None ; } if let Some (c) = char_get (self . s , self . b) { self . b += c . len_utf8 () ; } else { self . a = self . s . len () ; } Some (elt) } }
    };
}

impl_141!()