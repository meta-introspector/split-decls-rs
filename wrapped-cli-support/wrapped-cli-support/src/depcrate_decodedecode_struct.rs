// Generated macro for decode_struct (macro)
macro_rules! Depcrate_decodedecode_struct {
() => {
// Module: crate::decode
// Provides: {"decode_struct"}
// Dependencies: {}
macro_rules ! decode_struct { ($ name : ident ($ ($ lt : tt) *) $ ($ field : ident : $ ty : ty ,) *) => { pub struct $ name <$ ($ lt) *> { $ (pub $ field : $ ty ,) * } impl <'a > Decode <'a > for $ name <$ ($ lt) *> { fn decode (_data : & mut &'a [u8]) -> Self { log :: trace ! ("start decode `{}`" , stringify ! ($ name)) ; $ name { $ ($ field : Decode :: decode (_data) ,) * } } } } }
};
}
