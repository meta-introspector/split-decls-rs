// Generated macro for impl_131 (impl)
macro_rules! Depcrate_mapimpl_131 {
() => {
// Module: crate::map
// Provides: {"impl_131"}
// Dependencies: {}
impl < A , F , Req , Res > Clone for MapServiceFactory < A , F , Req , Res > where A : Clone , F : Clone , { fn clone (& self) -> Self { Self { a : self . a . clone () , f : self . f . clone () , r : PhantomData , } } }
};
}
