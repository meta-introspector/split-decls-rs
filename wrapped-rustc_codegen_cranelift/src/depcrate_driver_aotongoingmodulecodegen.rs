// Generated macro for OngoingModuleCodegen (enum)
macro_rules! Depcrate_driver_aotOngoingModuleCodegen {
() => {
// Module: crate::driver::aot
// Provides: {"OngoingModuleCodegen"}
// Dependencies: {}
enum OngoingModuleCodegen { Sync (Result < ModuleCodegenResult , String >) , Async (JoinHandle < Result < ModuleCodegenResult , String > >) , }
};
}
