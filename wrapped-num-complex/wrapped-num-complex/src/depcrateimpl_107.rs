// Generated macro for impl_107 (impl)
macro_rules! Depcrateimpl_107 {
() => {
// Module: crate
// Provides: {"impl_107"}
// Dependencies: {}
impl < T : Clone + Num > Add < T > for Complex < T > { type Output = Complex < T > ; # [inline] fn add (self , other : T) -> Self :: Output { Self :: Output :: new (self . re + other , self . im) } }
};
}
