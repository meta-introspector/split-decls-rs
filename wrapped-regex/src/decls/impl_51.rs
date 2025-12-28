macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'r , 'h > Iterator for Split < 'r , 'h > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { self . it . next () . map (| span | & self . haystack [span]) } }
    };
}

impl_51!();