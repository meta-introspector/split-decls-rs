macro_rules! deps {
    () => {
        ValuesRef!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        impl < 'a , T : 'a > DoubleEndedIterator for ValuesRef < 'a , T > { fn next_back (& mut self) -> Option < Self :: Item > { if let Some (next) = self . iter . next_back () { self . len -= 1 ; Some (next) } else { None } } }
    };
}

impl_455!();