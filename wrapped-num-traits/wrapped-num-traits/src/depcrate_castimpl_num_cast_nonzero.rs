// Generated macro for impl_num_cast_nonzero (macro)
macro_rules! Depcrate_castimpl_num_cast_nonzero {
() => {
// Module: crate::cast
// Provides: {"impl_num_cast_nonzero"}
// Dependencies: {}
macro_rules ! impl_num_cast_nonzero { ($ T : ty , $ conv : ident) => { impl NumCast for $ T { # [inline] fn from < N : ToPrimitive > (n : N) -> Option <$ T > { n .$ conv () . and_then (Self :: new) } } } ; }
};
}
