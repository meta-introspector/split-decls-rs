macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Intersection < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . by_ref () . rev () . find (| & nxt | self . other . contains (nxt)) } }
    };
}

impl_32!()