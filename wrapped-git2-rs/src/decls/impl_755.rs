macro_rules! deps {
    () => {
        StatusIter!();
        StatusEntry!();
    };
}

macro_rules! impl_755 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for StatusIter < 'a > { fn next_back (& mut self) -> Option < StatusEntry < 'a > > { self . range . next_back () . and_then (| i | self . statuses . get (i)) } }
    };
}

impl_755!()