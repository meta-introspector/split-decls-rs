macro_rules! deps {
    () => {
        Graphemes!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < 'a > Iterator for Graphemes < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { let (grapheme , size) = decode_grapheme (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; Some (grapheme) } }
    };
}

impl_178!();