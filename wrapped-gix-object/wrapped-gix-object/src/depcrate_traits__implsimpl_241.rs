// Generated macro for impl_241 (impl)
macro_rules! Depcrate_traits__implsimpl_241 {
() => {
// Module: crate::traits::_impls
// Provides: {"impl_241"}
// Dependencies: {}
impl < T > WriteTo for & T where T : WriteTo , { fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { < T as WriteTo > :: write_to (self , out) } fn kind (& self) -> Kind { < T as WriteTo > :: kind (self) } fn size (& self) -> u64 { < T as WriteTo > :: size (self) } }
};
}
