// Generated macro for ReproducibleArtifacts (struct)
macro_rules! Depcrate_core_build_steps_distReproducibleArtifacts {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"ReproducibleArtifacts"}
// Dependencies: {}
# [doc = " Tarball containing artifacts necessary to reproduce the build of rustc."] # [doc = ""] # [doc = " Currently this is the PGO (and possibly BOLT) profile data."] # [doc = ""] # [doc = " Should not be considered stable by end users."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct ReproducibleArtifacts { target : TargetSelection , }
};
}
