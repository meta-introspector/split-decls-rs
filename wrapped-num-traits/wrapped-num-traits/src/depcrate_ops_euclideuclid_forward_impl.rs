// Generated macro for euclid_forward_impl (macro)
macro_rules! Depcrate_ops_euclideuclid_forward_impl {
() => {
// Module: crate::ops::euclid
// Provides: {"euclid_forward_impl"}
// Dependencies: {}
macro_rules ! euclid_forward_impl { ($ ($ t : ty) *) => { $ (impl Euclid for $ t { # [inline] fn div_euclid (& self , v : &$ t) -> Self { <$ t >:: div_euclid (* self , * v) } # [inline] fn rem_euclid (& self , v : &$ t) -> Self { <$ t >:: rem_euclid (* self , * v) } }) * } }
};
}
