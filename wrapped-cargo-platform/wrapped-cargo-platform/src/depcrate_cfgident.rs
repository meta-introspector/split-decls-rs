// Generated macro for Ident (struct)
macro_rules! Depcrate_cfgIdent {
() => {
// Module: crate::cfg
// Provides: {"Ident"}
// Dependencies: {}
# [doc = " A identifier"] # [derive (Eq , Ord , PartialOrd , Clone , Debug)] pub struct Ident { # [doc = " The identifier"] pub name : String , # [doc = " Is this a raw ident: `r#async`"] # [doc = ""] # [doc = " It's mainly used for display and doesn't take"] # [doc = " part in the hash or equality (`foo` == `r#foo`)."] pub raw : bool , }
};
}
