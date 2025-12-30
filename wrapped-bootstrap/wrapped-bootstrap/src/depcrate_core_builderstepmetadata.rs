// Generated macro for StepMetadata (struct)
macro_rules! Depcrate_core_builderStepMetadata {
() => {
// Module: crate::core::builder
// Provides: {"StepMetadata"}
// Dependencies: {}
# [doc = " Metadata that describes an executed step, mostly for testing and tracing."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct StepMetadata { name : String , kind : Kind , target : TargetSelection , built_by : Option < Compiler > , stage : Option < u32 > , # [doc = " Additional opaque string printed in the metadata"] metadata : Option < String > , }
};
}
