// Generated macro for unique (macro)
macro_rules! Depcrate_allunique {
() => {
// Module: crate::all
// Provides: {"unique"}
// Dependencies: {}
macro_rules ! unique (($ (# [$ attr : meta]) * var =$ var : ident , mod =$ ($ module : ident) ::+, val =$ val : ident) => (unique ! ($ (# [$ attr]) * var =$ var , mod =$ ($ module) ::+, ty =$ val , val =$ val) ;) ; ($ (# [$ attr : meta]) * var =$ var : ident , mod =$ ($ module : ident) ::+, ty =$ ty : ident , val =$ val : ident) => ($ (# [$ attr]) * pub const $ var : &'static $ ($ module) ::+::$ ty = &$ ($ module) ::+::$ val ;) ;) ;
};
}
