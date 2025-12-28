macro_rules! deps {
    () => {
        OrderBy!();
        OrderByIter!();
    };
}

macro_rules! impl_559 {
    () => {
        deps!();
        impl < 'a > Iterator for OrderByIter < 'a > { type Item = OrderBy < 'a > ; # [inline] fn next (& mut self) -> Option < OrderBy < 'a > > { self . iter . next () . map (OrderBy) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_559!()