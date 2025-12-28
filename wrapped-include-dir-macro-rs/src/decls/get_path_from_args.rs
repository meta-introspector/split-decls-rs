macro_rules! get_path_from_args {
    () => {
        fn get_path_from_args (args : Vec < TokenTree >) -> Result < PathBuf , & 'static str > { match args . len () { 0 => Err ("empty") , 1 => { let nexttree = args . into_iter () . next () . unwrap () ; match nexttree { TokenTree :: Token (Token :: Literal (Lit :: Str (ref val , ..))) => Ok (val . into ()) , _ => Err ("not str") , } } _ => Err ("multiple trees") , } }
    };
}

get_path_from_args!();