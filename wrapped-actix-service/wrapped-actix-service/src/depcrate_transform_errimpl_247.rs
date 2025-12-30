// Generated macro for impl_247 (impl)
macro_rules! Depcrate_transform_errimpl_247 {
() => {
// Module: crate::transform_err
// Provides: {"impl_247"}
// Dependencies: {}
impl < T , S , Req , F , E > Clone for TransformMapInitErr < T , S , Req , F , E > where T : Clone , F : Clone , { fn clone (& self) -> Self { Self { transform : self . transform . clone () , mapper : self . mapper . clone () , _phantom : PhantomData , } } }
};
}
