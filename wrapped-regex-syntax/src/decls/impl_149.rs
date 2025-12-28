macro_rules! deps {
    () => {
        IntervalSetIter!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < 'a , I > Iterator for IntervalSetIter < 'a , I > { type Item = & 'a I ; fn next (& mut self) -> Option < & 'a I > { self . 0 . next () } }
    };
}

impl_149!()