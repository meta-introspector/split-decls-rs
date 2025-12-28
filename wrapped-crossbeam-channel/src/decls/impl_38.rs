macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . receiver . recv () . ok () } }
    };
}

impl_38!();