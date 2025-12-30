// Generated macro for impl_93 (impl)
macro_rules! Depcrateimpl_93 {
() => {
// Module: crate
// Provides: {"impl_93"}
// Dependencies: {}
impl < T : Clone + Num > Mul < Complex < T > > for Complex < T > { type Output = Self ; # [inline] fn mul (self , other : Self) -> Self :: Output { let re = self . re . clone () * other . re . clone () - self . im . clone () * other . im . clone () ; let im = self . re * other . im + self . im * other . re ; Self :: Output :: new (re , im) } }
};
}
