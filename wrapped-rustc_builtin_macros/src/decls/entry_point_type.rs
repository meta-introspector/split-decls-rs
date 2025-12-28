macro_rules! entry_point_type {
    () => {
        fn entry_point_type (item : & ast :: Item , at_root : bool) -> EntryPointType { match & item . kind { ast :: ItemKind :: Fn (fn_) => { rustc_ast :: entry :: entry_point_type (& item . attrs , at_root , Some (fn_ . ident . name)) } _ => EntryPointType :: None , } }
    };
}

entry_point_type!();