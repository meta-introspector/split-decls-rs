macro_rules! deps {
    () => {
        RefMulti!();
        Iter!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + 'a > Iterator for Iter < 'a , K > { type Item = RefMulti < 'a , K > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (RefMulti :: new) } }
    };
}

impl_22!();