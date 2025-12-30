// Generated macro for impl_124 (impl)
macro_rules! Depcrate_mapimpl_124 {
() => {
// Module: crate::map
// Provides: {"impl_124"}
// Dependencies: {}
impl < A , F , Req , Res > Clone for Map < A , F , Req , Res > where A : Clone , F : Clone , { fn clone (& self) -> Self { Map { service : self . service . clone () , f : self . f . clone () , _t : PhantomData , } } }
};
}
