macro_rules! deps {
    () => {
        TryIter!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T > Iterator for TryIter < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . receiver . try_recv () . ok () } }
    };
}

impl_34!();