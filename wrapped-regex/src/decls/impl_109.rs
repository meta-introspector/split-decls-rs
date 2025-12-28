macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'r , 'h > Iterator for Split < 'r , 'h > { type Item = & 'h str ; # [inline] fn next (& mut self) -> Option < & 'h str > { self . it . next () . map (| span | & self . haystack [span]) } }
    };
}

impl_109!();