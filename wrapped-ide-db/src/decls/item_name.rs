macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! item_name {
    () => {
        deps!();
        pub fn item_name (db : & RootDatabase , item : ItemInNs) -> Option < Name > { match item { ItemInNs :: Types (module_def_id) => module_def_id . name (db) , ItemInNs :: Values (module_def_id) => module_def_id . name (db) , ItemInNs :: Macros (macro_def_id) => Some (macro_def_id . name (db)) , } }
    };
}

item_name!();