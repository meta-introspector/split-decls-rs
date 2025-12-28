macro_rules! deps {
    () => {
        PathspecEntries!();
    };
}

macro_rules! impl_563 {
    () => {
        deps!();
        impl < 'list > DoubleEndedIterator for PathspecEntries < 'list > { fn next_back (& mut self) -> Option < & 'list [u8] > { self . range . next_back () . and_then (| i | self . list . entry (i)) } }
    };
}

impl_563!()