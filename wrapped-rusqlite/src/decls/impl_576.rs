macro_rules! deps {
    () => {
        ValueRef!();
        ValueIter!();
    };
}

macro_rules! impl_576 {
    () => {
        deps!();
        impl < 'a > Iterator for ValueIter < 'a > { type Item = ValueRef < 'a > ; # [inline] fn next (& mut self) -> Option < ValueRef < 'a > > { self . iter . next () . map (| & raw | unsafe { ValueRef :: from_value (raw) }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_576!();