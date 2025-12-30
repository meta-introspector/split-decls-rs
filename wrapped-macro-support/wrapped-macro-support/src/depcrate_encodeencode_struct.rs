// Generated macro for encode_struct (macro)
macro_rules! Depcrate_encodeencode_struct {
() => {
// Module: crate::encode
// Provides: {"encode_struct"}
// Dependencies: {}
macro_rules ! encode_struct { ($ name : ident ($ ($ lt : tt) *) $ ($ field : ident : $ ty : ty ,) *) => { struct $ name $ ($ lt) * { $ ($ field : $ ty ,) * } impl $ ($ lt) * Encode for $ name $ ($ lt) * { fn encode (& self , _dst : & mut Encoder) { $ (self .$ field . encode (_dst) ;) * } } } }
};
}
