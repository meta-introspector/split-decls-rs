macro_rules! deps {
    () => {
        Type!();
        DocLinkDef!();
    };
}

macro_rules! resolve_assoc_item {
    () => {
        deps!();
        fn resolve_assoc_item < 'db > (db : & 'db dyn HirDatabase , ty : & Type < 'db > , name : & Name , ns : Option < Namespace > ,) -> Option < DocLinkDef > { ty . iterate_assoc_items (db , move | assoc_item | { if assoc_item . name (db) ? != * name { return None ; } as_module_def_if_namespace_matches (assoc_item , ns) }) }
    };
}

resolve_assoc_item!()