// Generated macro for signed_impl (macro)
macro_rules! Depcrate_signsigned_impl {
() => {
// Module: crate::sign
// Provides: {"signed_impl"}
// Dependencies: {}
macro_rules ! signed_impl { ($ ($ t : ty) *) => ($ (impl Signed for $ t { # [inline] fn abs (& self) -> $ t { if self . is_negative () { -* self } else { * self } } # [inline] fn abs_sub (& self , other : &$ t) -> $ t { if * self <= * other { 0 } else { * self - * other } } # [inline] fn signum (& self) -> $ t { match * self { n if n > 0 => 1 , 0 => 0 , _ => - 1 , } } # [inline] fn is_positive (& self) -> bool { * self > 0 } # [inline] fn is_negative (& self) -> bool { * self < 0 } }) *) }
};
}
