macro_rules! deps {
    () => {
        DiffDelta!();
        Deltas!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < 'diff > DoubleEndedIterator for Deltas < 'diff > { fn next_back (& mut self) -> Option < DiffDelta < 'diff > > { self . range . next_back () . and_then (| i | self . diff . get_delta (i)) } }
    };
}

impl_339!();