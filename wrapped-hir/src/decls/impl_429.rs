macro_rules! deps {
    () => {
        Module!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl HasCrate for Module { fn krate (& self , _ : & dyn HirDatabase) -> Crate { Module :: krate (* self) } }
    };
}

impl_429!();