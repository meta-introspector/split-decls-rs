// Generated macro for impl_138 (impl)
macro_rules! Depcrate_edwards25519impl_138 {
() => {
// Module: crate::edwards25519
// Provides: {"impl_138"}
// Dependencies: {}
impl Sub < GePrecomp > for GeP3 { type Output = GeP1P1 ; fn sub (self , _rhs : GePrecomp) -> GeP1P1 { let y1_plus_x1 = self . y + self . x ; let y1_minus_x1 = self . y - self . x ; let a = y1_plus_x1 * _rhs . y_minus_x ; let b = y1_minus_x1 * _rhs . y_plus_x ; let c = _rhs . xy2d * self . t ; let d = self . z + self . z ; let x3 = a - b ; let y3 = a + b ; let z3 = d - c ; let t3 = d + c ; GeP1P1 { x : x3 , y : y3 , z : z3 , t : t3 , } } }
};
}
