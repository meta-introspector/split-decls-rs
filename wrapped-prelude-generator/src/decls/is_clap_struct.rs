macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! is_clap_struct {
    () => {
        deps!();
        fn is_clap_struct (item_struct : & ItemStruct) -> bool { for attr in & item_struct . attrs { if attr . path () . is_ident ("derive") { if let Ok (paths) = attr . parse_args_with (syn :: punctuated :: Punctuated :: < syn :: Path , syn :: Token ! [,] > :: parse_terminated ,) { for path in paths { if path . is_ident ("Parser") || path . is_ident ("Args") { return true ; } } } } } false }
    };
}

is_clap_struct!();