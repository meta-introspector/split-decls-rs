macro_rules! deps {
    () => {
        Type!();
        Crate!();
        HasCrate!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl HasCrate for Type < '_ > { fn krate (& self , _db : & dyn HirDatabase) -> Crate { self . env . krate . into () } }
    };
}

impl_207!()