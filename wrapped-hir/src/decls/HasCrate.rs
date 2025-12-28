macro_rules! deps {
    () => {
        Crate!();
        Trait!();
    };
}

macro_rules! HasCrate {
    () => {
        deps!();
        # [doc = " Trait for obtaining the defining crate of an item."] pub trait HasCrate { fn krate (& self , db : & dyn HirDatabase) -> Crate ; }
    };
}

HasCrate!()