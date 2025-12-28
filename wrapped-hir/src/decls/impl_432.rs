macro_rules! deps {
    () => {
        ItemContainer!();
        Crate!();
        HasContainer!();
        Module!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl HasContainer for Module { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { let def_map = self . id . def_map (db) ; match def_map [self . id . local_id] . parent { Some (parent_id) => ItemContainer :: Module (Module { id : def_map . module_id (parent_id) }) , None => ItemContainer :: Crate (def_map . krate () . into ()) , } } }
    };
}

impl_432!();