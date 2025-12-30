// Generated macro for impl_175 (impl)
macro_rules! Depcrate_map_init_errimpl_175 {
() => {
// Module: crate::map_init_err
// Provides: {"impl_175"}
// Dependencies: {}
impl < A , F , Req , E > Clone for MapInitErr < A , F , Req , E > where A : Clone , F : Clone , { fn clone (& self) -> Self { Self { a : self . a . clone () , f : self . f . clone () , e : PhantomData , } } }
};
}
