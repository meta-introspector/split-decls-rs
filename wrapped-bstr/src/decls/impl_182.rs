macro_rules! deps {
    () => {
        GraphemeIndices!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < 'a > Iterator for GraphemeIndices < 'a > { type Item = (usize , usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , usize , & 'a str) > { let index = self . forward_index ; let (grapheme , size) = decode_grapheme (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; self . forward_index += size ; Some ((index , index + size , grapheme)) } }
    };
}

impl_182!();