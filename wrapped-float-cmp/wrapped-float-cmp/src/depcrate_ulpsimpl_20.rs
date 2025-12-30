// Generated macro for impl_20 (impl)
macro_rules! Depcrate_ulpsimpl_20 {
() => {
// Module: crate::ulps
// Provides: {"impl_20"}
// Dependencies: {}
impl Ulps for f32 { type U = i32 ; fn ulps (& self , other : & f32) -> i32 { let ai32 : i32 = f32_ordered_bits (* self) as i32 ; let bi32 : i32 = f32_ordered_bits (* other) as i32 ; ai32 . wrapping_sub (bi32) } fn next (& self) -> Self { if self . is_infinite () && * self > 0.0 { * self } else if * self == - 0.0 && self . is_sign_negative () { 0.0 } else { let mut u = self . to_bits () ; if * self >= 0.0 { u += 1 ; } else { u -= 1 ; } f32 :: from_bits (u) } } fn prev (& self) -> Self { if self . is_infinite () && * self < 0.0 { * self } else if * self == 0.0 && self . is_sign_positive () { - 0.0 } else { let mut u = self . to_bits () ; if * self <= - 0.0 { u += 1 ; } else { u -= 1 ; } f32 :: from_bits (u) } } }
};
}
