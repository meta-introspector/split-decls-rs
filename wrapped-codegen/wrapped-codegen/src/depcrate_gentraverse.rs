// Generated macro for traverse (function)
macro_rules! Depcrate_gentraverse {
() => {
// Module: crate::gen
// Provides: {"traverse"}
// Dependencies: {}
pub fn traverse (defs : & Definitions , node : fn (& mut TokenStream , & mut TokenStream , & Node , & Definitions) ,) -> (TokenStream , TokenStream) { let mut types = defs . types . clone () ; for terminal in TERMINAL_TYPES { types . push (Node { ident : terminal . to_owned () , features : Features :: default () , data : Data :: Private , exhaustive : true , }) ; } types . sort_by (| a , b | a . ident . cmp (& b . ident)) ; let mut traits = TokenStream :: new () ; let mut impls = TokenStream :: new () ; for s in types { let features = cfg :: features (& s . features , DocCfg :: Ordinary) ; traits . extend (features . clone ()) ; impls . extend (features) ; node (& mut traits , & mut impls , & s , defs) ; } (traits , impls) }
};
}
