// Generated macro for impl_97 (impl)
macro_rules! Depcrateimpl_97 {
() => {
// Module: crate
// Provides: {"impl_97"}
// Dependencies: {}
impl < T : Clone + Num > Div < Complex < T > > for Complex < T > { type Output = Self ; # [inline] fn div (self , other : Self) -> Self :: Output { let norm_sqr = other . norm_sqr () ; let re = self . re . clone () * other . re . clone () + self . im . clone () * other . im . clone () ; let im = self . im * other . re - self . re * other . im ; Self :: Output :: new (re / norm_sqr . clone () , im / norm_sqr) } }
};
}
