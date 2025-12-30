// Generated macro for decode_api (macro)
macro_rules! Depcrate_decodedecode_api {
() => {
// Module: crate::decode
// Provides: {"decode_api"}
// Dependencies: {}
macro_rules ! decode_api { () => () ; (struct $ name : ident <'a > { $ ($ fields : tt) * } $ ($ rest : tt) *) => (decode_struct ! ($ name ('a) $ ($ fields) *) ; decode_api ! ($ ($ rest) *) ;) ; (struct $ name : ident { $ ($ fields : tt) * } $ ($ rest : tt) *) => (decode_struct ! ($ name () $ ($ fields) *) ; decode_api ! ($ ($ rest) *) ;) ; (enum $ name : ident <'a > { $ ($ variants : tt) * } $ ($ rest : tt) *) => (decode_enum ! ($ name ('a) $ ($ variants) *) ; decode_api ! ($ ($ rest) *) ;) ; (enum $ name : ident { $ ($ variants : tt) * } $ ($ rest : tt) *) => (decode_enum ! ($ name () $ ($ variants) *) ; decode_api ! ($ ($ rest) *) ;) ; }
};
}
