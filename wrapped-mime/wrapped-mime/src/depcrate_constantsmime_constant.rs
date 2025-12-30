// Generated macro for mime_constant (macro)
macro_rules! Depcrate_constantsmime_constant {
() => {
// Module: crate::constants
// Provides: {"mime_constant"}
// Dependencies: {}
macro_rules ! mime_constant { ($ kind : ident , $ id : ident , $ src : expr) => (mime_constant ! { @ DOC concat ! ("A `" , stringify ! ($ kind) , "` representing `\"" , $ src , "\"`.") , $ kind , $ id , $ src }) ; (@ DOC $ doc : expr , $ kind : ident , $ id : ident , $ src : expr) => (# [doc = $ doc] pub const $ id : $ kind = $ kind { mime : mime_parse :: constants ::$ id , } ;) }
};
}
