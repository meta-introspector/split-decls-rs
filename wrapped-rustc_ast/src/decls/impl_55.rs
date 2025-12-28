macro_rules! deps {
    () => {
        ByRef!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl ByRef { # [must_use] pub fn cap_ref_mutability (mut self , mutbl : Mutability) -> Self { if let ByRef :: Yes (old_mutbl) = & mut self { * old_mutbl = cmp :: min (* old_mutbl , mutbl) ; } self } }
    };
}

impl_55!();