macro_rules! deps {
    () => {
        ModuleDef!();
        HasCrate!();
        Crate!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl HasCrate for ModuleDef { fn krate (& self , db : & dyn HirDatabase) -> Crate { match self . module (db) { Some (module) => module . krate () , None => Crate :: core (db) . unwrap_or_else (| | db . all_crates () [0] . into ()) , } } }
    };
}

impl_235!()