macro_rules! deps {
    () => {
        OldestOrdered!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for OldestOrdered < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () } }
    };
}

impl_76!();