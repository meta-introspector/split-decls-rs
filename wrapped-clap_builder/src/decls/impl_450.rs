macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for Values < T > { fn next_back (& mut self) -> Option < Self :: Item > { if let Some (next) = self . iter . next_back () { self . len -= 1 ; Some (next) } else { None } } }
    };
}

impl_450!()