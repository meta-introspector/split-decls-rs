macro_rules! deps {
    () => {
        PathspecDiffEntries!();
        DiffDelta!();
    };
}

macro_rules! impl_566 {
    () => {
        deps!();
        impl < 'list > Iterator for PathspecDiffEntries < 'list > { type Item = DiffDelta < 'list > ; fn next (& mut self) -> Option < DiffDelta < 'list > > { self . range . next () . and_then (| i | self . list . diff_entry (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_566!();