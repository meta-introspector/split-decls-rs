macro_rules! deps {
    () => {
        ItemContainer!();
    };
}

macro_rules! HasContainer {
    () => {
        deps!();
        pub trait HasContainer { fn container (& self , db : & dyn HirDatabase) -> ItemContainer ; }
    };
}

HasContainer!()