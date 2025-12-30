// Generated macro for abort (macro)
macro_rules! Depcrateabort {
() => {
// Module: crate
// Provides: {"abort"}
// Dependencies: {}
macro_rules ! abort { ($ tokens : expr , $ message : expr $ (,) ?) => { return Err (syn :: Error :: new_spanned ($ tokens , $ message)) } ; }
};
}
