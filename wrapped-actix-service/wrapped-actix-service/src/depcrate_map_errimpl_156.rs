// Generated macro for impl_156 (impl)
macro_rules! Depcrate_map_errimpl_156 {
() => {
// Module: crate::map_err
// Provides: {"impl_156"}
// Dependencies: {}
impl < S , Req , F , E > Clone for MapErr < S , Req , F , E > where S : Clone , F : Clone , { fn clone (& self) -> Self { MapErr { service : self . service . clone () , mapper : self . mapper . clone () , _t : PhantomData , } } }
};
}
