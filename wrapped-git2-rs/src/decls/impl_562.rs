macro_rules! deps {
    () => {
        PathspecEntries!();
    };
}

macro_rules! impl_562 {
    () => {
        deps!();
        impl < 'list > Iterator for PathspecEntries < 'list > { type Item = & 'list [u8] ; fn next (& mut self) -> Option < & 'list [u8] > { self . range . next () . and_then (| i | self . list . entry (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_562!()