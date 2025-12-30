// Generated macro for pipeline_factory (function)
macro_rules! Depcrate_pipelinepipeline_factory {
() => {
// Module: crate::pipeline
// Provides: {"pipeline_factory"}
// Dependencies: {}
# [doc = " Construct new pipeline factory with one service factory."] pub (crate) fn pipeline_factory < I , SF , Req > (factory : I) -> PipelineFactory < SF , Req > where I : IntoServiceFactory < SF , Req > , SF : ServiceFactory < Req > , { PipelineFactory { factory : factory . into_factory () , _phantom : PhantomData , } }
};
}
