// Generated macro for TargetSpec (enum)
macro_rules! Depcrate_target_specTargetSpec {
() => {
// Module: crate::target_spec
// Provides: {"TargetSpec"}
// Dependencies: {}
# [doc = " A target represents a thing we can build or test."] # [doc = ""] # [doc = " We use it to calculate the CLI arguments required to build, run or"] # [doc = " test the target."] # [derive (Clone , Debug)] pub (crate) enum TargetSpec { Cargo (CargoTargetSpec) , ProjectJson (ProjectJsonTargetSpec) , }
};
}
