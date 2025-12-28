macro_rules! deps {
    () => {
        SplitN!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'r , 'h > Iterator for SplitN < 'r , 'h > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { self . it . next () . map (| span | & self . haystack [span]) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_54!()