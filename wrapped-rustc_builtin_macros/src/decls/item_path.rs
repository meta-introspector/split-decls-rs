macro_rules! item_path {
    () => {
        fn item_path (mod_path : & [Ident] , item_ident : & Ident) -> String { join_path_idents (mod_path . iter () . chain (iter :: once (item_ident))) }
    };
}

item_path!();