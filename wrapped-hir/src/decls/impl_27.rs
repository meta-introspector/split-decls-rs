macro_rules! deps {
    () => {
        HasVisibility!();
        Module!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl HasVisibility for Module { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { let def_map = self . id . def_map (db) ; let module_data = & def_map [self . id . local_id] ; module_data . visibility } }
    };
}

impl_27!()