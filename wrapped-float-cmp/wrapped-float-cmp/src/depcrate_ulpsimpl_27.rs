// Generated macro for impl_27 (impl)
macro_rules! Depcrate_ulpsimpl_27 {
() => {
// Module: crate::ulps
// Provides: {"impl_27"}
// Dependencies: {}
impl Ulps for f64 { type U = i64 ; fn ulps (& self , other : & f64) -> i64 { let ai64 : i64 = f64_ordered_bits (* self) as i64 ; let bi64 : i64 = f64_ordered_bits (* other) as i64 ; ai64 . wrapping_sub (bi64) } fn next (& self) -> Self { if self . is_infinite () && * self > 0.0 { * self } else if * self == - 0.0 && self . is_sign_negative () { 0.0 } else { let mut u = self . to_bits () ; if * self >= 0.0 { u += 1 ; } else { u -= 1 ; } f64 :: from_bits (u) } } fn prev (& self) -> Self { if self . is_infinite () && * self < 0.0 { * self } else if * self == 0.0 && self . is_sign_positive () { - 0.0 } else { let mut u = self . to_bits () ; if * self <= - 0.0 { u += 1 ; } else { u -= 1 ; } f64 :: from_bits (u) } } }
};
}
