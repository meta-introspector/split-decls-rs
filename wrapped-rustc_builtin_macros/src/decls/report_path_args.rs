macro_rules! deps {
    () => {
        DerivePathArgsList!();
        DerivePathArgsValue!();
    };
}

macro_rules! report_path_args {
    () => {
        deps!();
        fn report_path_args (sess : & Session , meta : & ast :: MetaItem) { let span = meta . span . with_lo (meta . path . span . hi ()) ; match meta . kind { MetaItemKind :: Word => { } MetaItemKind :: List (..) => { sess . dcx () . emit_err (errors :: DerivePathArgsList { span }) ; } MetaItemKind :: NameValue (..) => { sess . dcx () . emit_err (errors :: DerivePathArgsValue { span }) ; } } }
    };
}

report_path_args!()