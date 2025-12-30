// Generated macro for encode_enum (macro)
macro_rules! Depcrate_encodeencode_enum {
() => {
// Module: crate::encode
// Provides: {"encode_enum"}
// Dependencies: {}
macro_rules ! encode_enum { ($ name : ident ($ ($ lt : tt) *) $ ($ fields : tt) *) => (enum $ name $ ($ lt) * { $ ($ fields) * } impl $ ($ lt) * Encode for $ name $ ($ lt) * { fn encode (& self , dst : & mut Encoder) { use self ::$ name ::*; encode_enum ! (@ arms self dst (0) () $ ($ fields) *) } }) ; (@ arms $ me : ident $ dst : ident ($ cnt : expr) ($ ($ arms : tt) *)) => (encode_enum ! (@ expr match $ me { $ ($ arms) * })) ; (@ arms $ me : ident $ dst : ident ($ cnt : expr) ($ ($ arms : tt) *) $ name : ident , $ ($ rest : tt) *) => (encode_enum ! (@ arms $ me $ dst ($ cnt + 1) ($ ($ arms) * $ name => $ dst . byte ($ cnt) ,) $ ($ rest) *)) ; (@ arms $ me : ident $ dst : ident ($ cnt : expr) ($ ($ arms : tt) *) $ name : ident ($ t : ty) , $ ($ rest : tt) *) => (encode_enum ! (@ arms $ me $ dst ($ cnt + 1) ($ ($ arms) * $ name (val) => { $ dst . byte ($ cnt) ; val . encode ($ dst) }) $ ($ rest) *)) ; (@ expr $ e : expr) => ($ e) ; }
};
}
