// Generated macro for impl_into (macro)
macro_rules! Depcrate_x86_64impl_into {
() => {
// Module: crate::x86_64
// Provides: {"impl_into"}
// Dependencies: {}
macro_rules ! impl_into { ($ storage : ident , $ array : ty , $ name : ident) => { impl From <$ storage > for $ array { # [inline (always)] fn from (vec : $ storage) -> Self { unsafe { vec .$ name } } } } ; }
};
}
