macro_rules! deps {
    () => {
        PathspecFailedEntries!();
    };
}

macro_rules! impl_570 {
    () => {
        deps!();
        impl < 'list > Iterator for PathspecFailedEntries < 'list > { type Item = & 'list [u8] ; fn next (& mut self) -> Option < & 'list [u8] > { self . range . next () . and_then (| i | self . list . failed_entry (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_570!()