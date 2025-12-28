macro_rules! deps {
    () => {
        HasContainer!();
        ItemContainer!();
        Crate!();
        Module!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl HasContainer for Module { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { let def_map = self . id . def_map (db) ; match def_map [self . id . local_id] . parent { Some (parent_id) => ItemContainer :: Module (Module { id : def_map . module_id (parent_id) }) , None => ItemContainer :: Crate (def_map . krate () . into ()) , } } }
    };
}

impl_216!()