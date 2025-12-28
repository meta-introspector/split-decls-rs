macro_rules! deps {
    () => {
        UnitRef!();
        Reader!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < 'a , R : Reader > Clone for UnitRef < 'a , R > { fn clone (& self) -> Self { * self } }
    };
}

impl_273!()