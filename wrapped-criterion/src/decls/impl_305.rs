macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        # [allow (clippy :: expl_impl_clone_on_copy)] impl < 'a , X , Y > Clone for Data < 'a , X , Y > { fn clone (& self) -> Data < 'a , X , Y > { * self } }
    };
}

impl_305!()