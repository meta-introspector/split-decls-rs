// Generated macro for impl_191 (impl)
macro_rules! Depcrate_pipelineimpl_191 {
() => {
// Module: crate::pipeline
// Provides: {"impl_191"}
// Dependencies: {}
impl < T , Req > Clone for PipelineFactory < T , Req > where T : Clone , { fn clone (& self) -> Self { PipelineFactory { factory : self . factory . clone () , _phantom : PhantomData , } } }
};
}
