// Generated macro for DeriveItemArgsOption (enum)
macro_rules! Depcrate_item_typeDeriveItemArgsOption {
() => {
// Module: crate::item_type
// Provides: {"DeriveItemArgsOption"}
// Dependencies: {}
# [derive (Parse , Debug)] enum DeriveItemArgsOption { Some { # [parse (peek)] # [to_tokens ("(")] _paren : token :: Paren , args : DeriveItemArgs , } , None , }
};
}
