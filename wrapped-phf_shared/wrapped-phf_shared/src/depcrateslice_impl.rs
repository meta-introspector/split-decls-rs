// Generated macro for slice_impl (macro)
macro_rules! Depcrateslice_impl {
() => {
// Module: crate
// Provides: {"slice_impl"}
// Dependencies: {}
macro_rules ! slice_impl (($ t : ty) => { impl PhfHash for [$ t] { # [inline] fn phf_hash < H : Hasher > (& self , state : & mut H) { for v in self { v . phf_hash (state) ; } } } } ;) ;
};
}
