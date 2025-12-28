macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Difference < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . by_ref () . rev () . find (| & nxt | ! self . other . contains (nxt)) } }
    };
}

impl_93!()