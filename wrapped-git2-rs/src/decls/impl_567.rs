macro_rules! deps {
    () => {
        PathspecDiffEntries!();
        DiffDelta!();
    };
}

macro_rules! impl_567 {
    () => {
        deps!();
        impl < 'list > DoubleEndedIterator for PathspecDiffEntries < 'list > { fn next_back (& mut self) -> Option < DiffDelta < 'list > > { self . range . next_back () . and_then (| i | self . list . diff_entry (i)) } }
    };
}

impl_567!()