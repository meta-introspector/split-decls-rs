// Generated macro for encode_api (macro)
macro_rules! Depcrate_encodeencode_api {
() => {
// Module: crate::encode
// Provides: {"encode_api"}
// Dependencies: {}
macro_rules ! encode_api { () => () ; (struct $ name : ident <'a > { $ ($ fields : tt) * } $ ($ rest : tt) *) => (encode_struct ! ($ name (<'a >) $ ($ fields) *) ; encode_api ! ($ ($ rest) *) ;) ; (struct $ name : ident { $ ($ fields : tt) * } $ ($ rest : tt) *) => (encode_struct ! ($ name () $ ($ fields) *) ; encode_api ! ($ ($ rest) *) ;) ; (enum $ name : ident <'a > { $ ($ variants : tt) * } $ ($ rest : tt) *) => (encode_enum ! ($ name (<'a >) $ ($ variants) *) ; encode_api ! ($ ($ rest) *) ;) ; (enum $ name : ident { $ ($ variants : tt) * } $ ($ rest : tt) *) => (encode_enum ! ($ name () $ ($ variants) *) ; encode_api ! ($ ($ rest) *) ;) ; }
};
}
