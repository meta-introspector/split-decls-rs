macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < LenT : LenType > Iterator for Drain < '_ , LenT > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn last (mut self) -> Option < char > { self . next_back () } }
    };
}

impl_215!();