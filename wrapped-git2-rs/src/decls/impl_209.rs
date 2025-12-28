macro_rules! deps {
    () => {
        BlameHunk!();
        BlameIter!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'blame > Iterator for BlameIter < 'blame > { type Item = BlameHunk < 'blame > ; fn next (& mut self) -> Option < BlameHunk < 'blame > > { self . range . next () . and_then (| i | self . blame . get_index (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_209!();