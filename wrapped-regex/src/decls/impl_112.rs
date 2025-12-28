macro_rules! deps {
    () => {
        SplitN!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < 'r , 'h > Iterator for SplitN < 'r , 'h > { type Item = & 'h str ; # [inline] fn next (& mut self) -> Option < & 'h str > { self . it . next () . map (| span | & self . haystack [span]) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_112!();