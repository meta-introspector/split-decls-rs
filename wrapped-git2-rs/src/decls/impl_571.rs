macro_rules! deps {
    () => {
        PathspecFailedEntries!();
    };
}

macro_rules! impl_571 {
    () => {
        deps!();
        impl < 'list > DoubleEndedIterator for PathspecFailedEntries < 'list > { fn next_back (& mut self) -> Option < & 'list [u8] > { self . range . next_back () . and_then (| i | self . list . failed_entry (i)) } }
    };
}

impl_571!()