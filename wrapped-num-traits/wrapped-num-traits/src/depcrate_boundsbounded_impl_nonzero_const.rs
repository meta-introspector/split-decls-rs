// Generated macro for bounded_impl_nonzero_const (macro)
macro_rules! Depcrate_boundsbounded_impl_nonzero_const {
() => {
// Module: crate::bounds
// Provides: {"bounded_impl_nonzero_const"}
// Dependencies: {}
macro_rules ! bounded_impl_nonzero_const { ($ t : ty , $ v : expr , $ i : ident) => { const $ i : $ t = match <$ t >:: new ($ v) { Some (nz) => nz , None => panic ! ("bad nonzero bound!") , } ; } ; }
};
}
