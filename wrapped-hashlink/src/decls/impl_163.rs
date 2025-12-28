macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < K > Iterator for IntoIter < K > { type Item = K ; # [inline] fn next (& mut self) -> Option < K > { self . iter . next () . map (| (k , _) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_163!();