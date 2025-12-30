// Generated macro for Pipeline (struct)
macro_rules! Depcrate_pipelinePipeline {
() => {
// Module: crate::pipeline
// Provides: {"Pipeline"}
// Dependencies: {}
# [doc = " Pipeline service - pipeline allows to compose multiple service into one service."] pub (crate) struct Pipeline < S , Req > { service : S , _phantom : PhantomData < fn (Req) > , }
};
}
