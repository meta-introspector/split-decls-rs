macro_rules! deps {
    () => {
        GraphemeIndices!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for GraphemeIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < (usize , usize , & 'a str) > { let (grapheme , size) = decode_last_grapheme (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [.. self . bs . len () - size] ; self . reverse_index -= size ; Some ((self . reverse_index , self . reverse_index + size , grapheme)) } }
    };
}

impl_183!();