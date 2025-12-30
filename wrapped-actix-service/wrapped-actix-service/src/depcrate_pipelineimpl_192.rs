// Generated macro for impl_192 (impl)
macro_rules! Depcrate_pipelineimpl_192 {
() => {
// Module: crate::pipeline
// Provides: {"impl_192"}
// Dependencies: {}
impl < SF , Req > ServiceFactory < Req > for PipelineFactory < SF , Req > where SF : ServiceFactory < Req > , { type Config = SF :: Config ; type Response = SF :: Response ; type Error = SF :: Error ; type Service = SF :: Service ; type InitError = SF :: InitError ; type Future = SF :: Future ; # [inline] fn new_service (& self , cfg : SF :: Config) -> Self :: Future { self . factory . new_service (cfg) } }
};
}
