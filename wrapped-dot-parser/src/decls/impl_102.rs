macro_rules! deps {
    () => {
        EdgeSet!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < A > EdgeSet < A > { fn empty () -> Self { Self { set : Vec :: new () } } fn map < F , B > (self , f : F) -> EdgeSet < B > where F : Fn (A) -> Option < B > , { let new_set = self . set . into_iter () . map (| edge | edge . map (& f)) . collect () ; EdgeSet { set : new_set } } }
    };
}

impl_102!()