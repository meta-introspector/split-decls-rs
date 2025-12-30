// Generated macro for forward_writes (macro)
macro_rules! Depcrate_hasherforward_writes {
() => {
// Module: crate::hasher
// Provides: {"forward_writes"}
// Dependencies: {}
# [cfg (feature = "default-hasher")] macro_rules ! forward_writes { ($ ($ write : ident ($ ty : ty) ,) *) => { $ (# [inline (always)] fn $ write (& mut self , arg : $ ty) { self . inner .$ write (arg) ; }) * } }
};
}
