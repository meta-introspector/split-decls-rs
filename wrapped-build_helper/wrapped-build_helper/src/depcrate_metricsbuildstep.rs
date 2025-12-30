// Generated macro for BuildStep (struct)
macro_rules! Depcrate_metricsBuildStep {
() => {
// Module: crate::metrics
// Provides: {"BuildStep"}
// Dependencies: {}
# [doc = " Represents a single bootstrap step, with the accumulated duration of all its children."] # [derive (Clone , Debug)] pub struct BuildStep { pub r#type : String , pub children : Vec < BuildStep > , pub duration : Duration , pub full_name : String , }
};
}
