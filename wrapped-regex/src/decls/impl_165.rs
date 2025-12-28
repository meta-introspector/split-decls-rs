macro_rules! deps {
    () => {
        SetMatchesIter!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < 'a > Iterator for SetMatchesIter < 'a > { type Item = usize ; fn next (& mut self) -> Option < usize > { self . 0 . next () . map (| pid | pid . as_usize ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
    };
}

impl_165!();