macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < K > Iterator for Drain < '_ , K > { type Item = K ; # [inline] fn next (& mut self) -> Option < K > { self . iter . next () . map (| (k , _) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_166!();