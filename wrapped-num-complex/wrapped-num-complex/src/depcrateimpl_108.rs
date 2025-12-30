// Generated macro for impl_108 (impl)
macro_rules! Depcrateimpl_108 {
() => {
// Module: crate
// Provides: {"impl_108"}
// Dependencies: {}
impl < T : Clone + Num > Sub < T > for Complex < T > { type Output = Complex < T > ; # [inline] fn sub (self , other : T) -> Self :: Output { Self :: Output :: new (self . re - other , self . im) } }
};
}
