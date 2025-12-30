// Generated macro for get_path_from_args (function)
macro_rules! Depcrateget_path_from_args {
() => {
// Module: crate
// Provides: {"get_path_from_args"}
// Dependencies: {}
fn get_path_from_args (args : Vec < TokenTree >) -> Result < PathBuf , & 'static str > { match args . len () { 0 => Err ("empty") , 1 => { let nexttree = args . into_iter () . next () . unwrap () ; match nexttree { TokenTree :: Token (Token :: Literal (Lit :: Str (ref val , ..))) => Ok (val . into ()) , _ => Err ("not str") , } } _ => Err ("multiple trees") , } }
};
}
