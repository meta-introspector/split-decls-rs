macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a > Iterator for Iter < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { self . inner . next () . map (| component | component . as_str ()) } }
    };
}

impl_53!();