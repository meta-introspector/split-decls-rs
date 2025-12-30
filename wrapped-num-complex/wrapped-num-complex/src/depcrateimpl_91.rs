// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl < T : Clone + Num > Sub < Complex < T > > for Complex < T > { type Output = Self ; # [inline] fn sub (self , other : Self) -> Self :: Output { Self :: Output :: new (self . re - other . re , self . im - other . im) } }
};
}
