// Generated macro for compute_2x2_uptrig_svd (function)
macro_rules! Depcrate_linalg_svdcompute_2x2_uptrig_svd {
() => {
// Module: crate::linalg::svd
// Provides: {"compute_2x2_uptrig_svd"}
// Dependencies: {}
fn compute_2x2_uptrig_svd < T : RealField > (m11 : T , m12 : T , m22 : T , compute_u : bool , compute_v : bool ,) -> (Option < GivensRotation < T > > , Vector2 < T > , Option < GivensRotation < T > > ,) { let two : T :: RealField = crate :: convert (2.0f64) ; let half : T :: RealField = crate :: convert (0.5f64) ; let denom = (m11 . clone () + m22 . clone ()) . hypot (m12 . clone ()) + (m11 . clone () - m22 . clone ()) . hypot (m12 . clone ()) ; let mut v1 = m11 . clone () * m22 . clone () * two / denom . clone () ; let mut v2 = half * denom ; let mut u = None ; let mut v_t = None ; if compute_u || compute_v { let (csv , sgn_v) = GivensRotation :: new (m11 . clone () * m12 . clone () , v1 . clone () * v1 . clone () - m11 . clone () * m11 . clone () ,) ; v1 *= sgn_v . clone () ; v2 *= sgn_v ; if compute_v { v_t = Some (csv . clone ()) ; } let cu = (m11 . scale (csv . c ()) + m12 * csv . s ()) / v1 . clone () ; let su = (m22 * csv . s ()) / v1 . clone () ; let (csu , sgn_u) = GivensRotation :: new (cu , su) ; v1 *= sgn_u . clone () ; v2 *= sgn_u ; if compute_u { u = Some (csu) ; } } (u , Vector2 :: new (v1 , v2) , v_t) }
};
}
