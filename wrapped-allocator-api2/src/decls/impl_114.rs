macro_rules! deps {
    () => {
        Allocator!();
        Drain!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < T , A : Allocator > Iterator for Drain < '_ , T , A > { type Item = T ; # [inline (always)] fn next (& mut self) -> Option < T > { self . iter . next () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_114!();