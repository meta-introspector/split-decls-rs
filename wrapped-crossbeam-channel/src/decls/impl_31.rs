macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > Iterator for Iter < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . receiver . recv () . ok () } }
    };
}

impl_31!()