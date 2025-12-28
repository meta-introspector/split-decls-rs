macro_rules! deps {
    () => {
        HasCrate!();
        Type!();
        Crate!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl HasCrate for Type < '_ > { fn krate (& self , _db : & dyn HirDatabase) -> Crate { self . env . krate . into () } }
    };
}

impl_423!()