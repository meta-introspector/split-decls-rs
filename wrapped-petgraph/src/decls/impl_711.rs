macro_rules! deps {
    () => {
        IndexType!();
        EdgeWeightsMut!();
    };
}

macro_rules! impl_711 {
    () => {
        deps!();
        impl < 'a , E , Ix > Iterator for EdgeWeightsMut < 'a , E , Ix > where Ix : IndexType , { type Item = & 'a mut E ; fn next (& mut self) -> Option < & 'a mut E > { self . edges . next () . map (| edge | & mut edge . weight) } fn size_hint (& self) -> (usize , Option < usize >) { self . edges . size_hint () } }
    };
}

impl_711!()