// Generated macro for unexpected_token (macro)
macro_rules! Depcrateunexpected_token {
() => {
// Module: crate
// Provides: {"unexpected_token"}
// Dependencies: {}
macro_rules ! unexpected_token { ($ item : expr , $ msg : expr) => { if let Some (i) = $ item { bail ! (i , "unexpected {}" , $ msg) ; } } ; }
};
}
