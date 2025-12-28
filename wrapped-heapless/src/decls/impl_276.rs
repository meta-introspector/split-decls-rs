macro_rules! deps {
    () => {
        Drain!();
        LenType!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < T , LenT : LenType > Iterator for Drain < '_ , T , LenT > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . iter . next () . map (| elt | unsafe { ptr :: read (core :: ptr :: from_ref (elt)) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_276!();