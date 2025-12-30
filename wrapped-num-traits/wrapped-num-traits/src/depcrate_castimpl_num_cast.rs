// Generated macro for impl_num_cast (macro)
macro_rules! Depcrate_castimpl_num_cast {
() => {
// Module: crate::cast
// Provides: {"impl_num_cast"}
// Dependencies: {}
macro_rules ! impl_num_cast { ($ T : ty , $ conv : ident) => { impl NumCast for $ T { # [inline] fn from < N : ToPrimitive > (n : N) -> Option <$ T > { n .$ conv () } } } ; }
};
}
