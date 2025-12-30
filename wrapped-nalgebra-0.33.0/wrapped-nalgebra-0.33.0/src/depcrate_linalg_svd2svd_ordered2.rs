// Generated macro for svd_ordered2 (function)
macro_rules! Depcrate_linalg_svd2svd_ordered2 {
() => {
// Module: crate::linalg::svd2
// Provides: {"svd_ordered2"}
// Dependencies: {}
pub fn svd_ordered2 < T : RealField > (m : & Matrix2 < T > , compute_u : bool , compute_v : bool ,) -> SVD < T , U2 , U2 > { let half : T = crate :: convert (0.5) ; let one : T = crate :: convert (1.0) ; let e = (m . m11 . clone () + m . m22 . clone ()) * half . clone () ; let f = (m . m11 . clone () - m . m22 . clone ()) * half . clone () ; let g = (m . m21 . clone () + m . m12 . clone ()) * half . clone () ; let h = (m . m21 . clone () - m . m12 . clone ()) * half . clone () ; let q = (e . clone () * e . clone () + h . clone () * h . clone ()) . sqrt () ; let r = (f . clone () * f . clone () + g . clone () * g . clone ()) . sqrt () ; let sx = q . clone () + r . clone () ; let sy = q - r ; let sy_sign = if sy < T :: zero () { - one } else { one } ; let singular_values = Vector2 :: new (sx , sy * sy_sign . clone ()) ; if compute_u || compute_v { let a1 = g . atan2 (f) ; let a2 = h . atan2 (e) ; let theta = (a2 . clone () - a1 . clone ()) * half . clone () ; let phi = (a2 + a1) * half ; let (st , ct) = theta . sin_cos () ; let (sp , cp) = phi . sin_cos () ; let u = Matrix2 :: new (cp . clone () , - sp . clone () , sp , cp) ; let v_t = Matrix2 :: new (ct . clone () , - st . clone () , st * sy_sign . clone () , ct * sy_sign) ; SVD { u : if compute_u { Some (u) } else { None } , singular_values , v_t : if compute_v { Some (v_t) } else { None } , } } else { SVD { u : None , singular_values , v_t : None , } } }
};
}
