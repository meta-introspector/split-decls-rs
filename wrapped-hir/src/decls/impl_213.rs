macro_rules! deps {
    () => {
        Crate!();
        Module!();
        HasCrate!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl HasCrate for Module { fn krate (& self , _ : & dyn HirDatabase) -> Crate { Module :: krate (* self) } }
    };
}

impl_213!()