macro_rules! container_name {
    () => {
        fn container_name (db : & RootDatabase , t : impl HasContainer) -> Option < Symbol > { match t . container (db) { hir :: ItemContainer :: Trait (it) => Some (it . name (db) . symbol () . clone ()) , hir :: ItemContainer :: Module (it) => it . name (db) . map (| name | name . symbol () . clone ()) , _ => None , } }
    };
}

container_name!();