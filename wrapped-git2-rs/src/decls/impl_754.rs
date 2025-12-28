macro_rules! deps {
    () => {
        StatusIter!();
        StatusEntry!();
    };
}

macro_rules! impl_754 {
    () => {
        deps!();
        impl < 'a > Iterator for StatusIter < 'a > { type Item = StatusEntry < 'a > ; fn next (& mut self) -> Option < StatusEntry < 'a > > { self . range . next () . and_then (| i | self . statuses . get (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_754!()