macro_rules! deps {
    () => {
        Graphemes!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Graphemes < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a str > { let (grapheme , size) = decode_last_grapheme (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [.. self . bs . len () - size] ; Some (grapheme) } }
    };
}

impl_179!()