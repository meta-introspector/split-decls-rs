macro_rules! deps {
    () => {
        Slots!();
        SlotsIter!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Iterator for SlotsIter { type Item = usize ; fn next (& mut self) -> Option < usize > { let slot = self . slots . 0 . trailing_zeros () . as_usize () ; if slot >= Slots :: LIMIT { return None ; } self . slots = self . slots . remove (slot) ; Some (slot) } }
    };
}

impl_93!()