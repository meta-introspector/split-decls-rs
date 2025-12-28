macro_rules! deps {
    () => {
        DefDatabase!();
        HasSource!();
    };
}

macro_rules! any_has_attrs {
    () => {
        deps!();
        fn any_has_attrs < 'db > (db : & (dyn DefDatabase + 'db) , id : impl Lookup < Database = dyn DefDatabase , Data = impl HasSource < Value = impl ast :: HasAttrs > > ,) -> InFile < ast :: AnyHasAttrs > { id . lookup (db) . source (db) . map (ast :: AnyHasAttrs :: new) }
    };
}

any_has_attrs!();