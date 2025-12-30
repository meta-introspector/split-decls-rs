// Generated macro for decode_enum (macro)
macro_rules! Depcrate_decodedecode_enum {
() => {
// Module: crate::decode
// Provides: {"decode_enum"}
// Dependencies: {}
macro_rules ! decode_enum { ($ name : ident ($ ($ lt : tt) *) $ ($ fields : tt) *) => (pub enum $ name <$ ($ lt) *> { $ ($ fields) * } impl <'a > Decode <'a > for $ name <$ ($ lt) *> { fn decode (data : & mut &'a [u8]) -> Self { use self ::$ name ::*; decode_enum ! (@ arms data dst (0) () $ ($ fields) *) } }) ; (@ arms $ data : ident $ dst : ident ($ cnt : expr) ($ ($ arms : tt) *)) => (decode_enum ! (@ expr match get ($ data) { $ ($ arms) * _ => unreachable ! () })) ; (@ arms $ data : ident $ dst : ident ($ cnt : expr) ($ ($ arms : tt) *) $ name : ident , $ ($ rest : tt) *) => (decode_enum ! (@ arms $ data $ dst ($ cnt + 1) ($ ($ arms) * n if n == $ cnt => $ name ,) $ ($ rest) *)) ; (@ arms $ data : ident $ dst : ident ($ cnt : expr) ($ ($ arms : tt) *) $ name : ident ($ t : ty) , $ ($ rest : tt) *) => (decode_enum ! (@ arms $ data $ dst ($ cnt + 1) ($ ($ arms) * n if n == $ cnt => $ name (Decode :: decode ($ data)) ,) $ ($ rest) *)) ; (@ expr $ e : expr) => ($ e) ; }
};
}
