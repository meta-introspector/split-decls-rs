macro_rules! deps {
    () => {
        Idx!();
        MixedBitIter!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a , T : Idx > Iterator for MixedBitIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { match self { MixedBitIter :: Small (iter) => iter . next () , MixedBitIter :: Large (iter) => iter . next () , } } }
    };
}

impl_46!()