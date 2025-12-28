macro_rules! deps {
    () => {
        Lines!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < 'i > Iterator for Lines < 'i > { type Item = & 'i str ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| span | span . as_str ()) } }
    };
}

impl_161!()