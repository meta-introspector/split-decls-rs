macro_rules! deps {
    () => {
        EdgeWeights!();
        IndexType!();
    };
}

macro_rules! impl_709 {
    () => {
        deps!();
        impl < 'a , E , Ix > Iterator for EdgeWeights < 'a , E , Ix > where Ix : IndexType , { type Item = & 'a E ; fn next (& mut self) -> Option < & 'a E > { self . edges . next () . map (| edge | & edge . weight) } fn size_hint (& self) -> (usize , Option < usize >) { self . edges . size_hint () } }
    };
}

impl_709!();