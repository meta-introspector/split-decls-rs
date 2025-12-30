// Generated macro for impl_163 (impl)
macro_rules! Depcrate_map_errimpl_163 {
() => {
// Module: crate::map_err
// Provides: {"impl_163"}
// Dependencies: {}
impl < SF , Req , F , E > Clone for MapErrServiceFactory < SF , Req , F , E > where SF : ServiceFactory < Req > + Clone , F : Fn (SF :: Error) -> E + Clone , { fn clone (& self) -> Self { Self { a : self . a . clone () , f : self . f . clone () , e : PhantomData , } } }
};
}
