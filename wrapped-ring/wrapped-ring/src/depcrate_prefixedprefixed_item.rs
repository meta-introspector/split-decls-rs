// Generated macro for prefixed_item (macro)
macro_rules! Depcrate_prefixedprefixed_item {
() => {
// Module: crate::prefixed
// Provides: {"prefixed_item"}
// Dependencies: {}
macro_rules ! prefixed_item { { $ attr : ident $ name : ident { $ item : item } } => { # [$ attr = concat ! (prefix ! () , stringify ! ($ name))] $ item } ; }
};
}
