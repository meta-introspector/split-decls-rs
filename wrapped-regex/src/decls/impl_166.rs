macro_rules! deps {
    () => {
        SetMatchesIter!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for SetMatchesIter < 'a > { fn next_back (& mut self) -> Option < usize > { self . 0 . next_back () . map (| pid | pid . as_usize ()) } }
    };
}

impl_166!()