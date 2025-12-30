// Generated macro for checked_euclid_forward_impl (macro)
macro_rules! Depcrate_ops_euclidchecked_euclid_forward_impl {
() => {
// Module: crate::ops::euclid
// Provides: {"checked_euclid_forward_impl"}
// Dependencies: {}
macro_rules ! checked_euclid_forward_impl { ($ ($ t : ty) *) => { $ (impl CheckedEuclid for $ t { # [inline] fn checked_div_euclid (& self , v : &$ t) -> Option < Self > { <$ t >:: checked_div_euclid (* self , * v) } # [inline] fn checked_rem_euclid (& self , v : &$ t) -> Option < Self > { <$ t >:: checked_rem_euclid (* self , * v) } }) * } }
};
}
