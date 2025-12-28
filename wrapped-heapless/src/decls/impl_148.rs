macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (k , _) | k) } }
    };
}

impl_148!();