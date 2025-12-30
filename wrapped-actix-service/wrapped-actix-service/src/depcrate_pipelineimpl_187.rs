// Generated macro for impl_187 (impl)
macro_rules! Depcrate_pipelineimpl_187 {
() => {
// Module: crate::pipeline
// Provides: {"impl_187"}
// Dependencies: {}
impl < T , Req > Clone for Pipeline < T , Req > where T : Clone , { fn clone (& self) -> Self { Pipeline { service : self . service . clone () , _phantom : PhantomData , } } }
};
}
