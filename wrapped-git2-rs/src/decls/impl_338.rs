macro_rules! deps {
    () => {
        DiffDelta!();
        Deltas!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl < 'diff > Iterator for Deltas < 'diff > { type Item = DiffDelta < 'diff > ; fn next (& mut self) -> Option < DiffDelta < 'diff > > { self . range . next () . and_then (| i | self . diff . get_delta (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_338!();