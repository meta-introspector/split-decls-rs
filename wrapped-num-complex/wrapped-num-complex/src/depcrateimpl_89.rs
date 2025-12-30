// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl < T : Clone + Num > Add < Complex < T > > for Complex < T > { type Output = Self ; # [inline] fn add (self , other : Self) -> Self :: Output { Self :: Output :: new (self . re + other . re , self . im + other . im) } }
};
}
