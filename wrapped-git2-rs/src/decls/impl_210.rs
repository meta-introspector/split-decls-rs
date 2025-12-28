macro_rules! deps {
    () => {
        BlameIter!();
        BlameHunk!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < 'blame > DoubleEndedIterator for BlameIter < 'blame > { fn next_back (& mut self) -> Option < BlameHunk < 'blame > > { self . range . next_back () . and_then (| i | self . blame . get_index (i)) } }
    };
}

impl_210!()