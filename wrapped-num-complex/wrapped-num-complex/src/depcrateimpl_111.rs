// Generated macro for impl_111 (impl)
macro_rules! Depcrateimpl_111 {
() => {
// Module: crate
// Provides: {"impl_111"}
// Dependencies: {}
impl < T : Clone + Num > Rem < T > for Complex < T > { type Output = Complex < T > ; # [inline] fn rem (self , other : T) -> Self :: Output { Self :: Output :: new (self . re % other . clone () , self . im % other) } }
};
}
