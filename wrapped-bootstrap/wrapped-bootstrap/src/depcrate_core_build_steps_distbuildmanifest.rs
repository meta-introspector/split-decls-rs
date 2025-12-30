// Generated macro for BuildManifest (struct)
macro_rules! Depcrate_core_build_steps_distBuildManifest {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"BuildManifest"}
// Dependencies: {}
# [doc = " Tarball containing a prebuilt version of the build-manifest tool, intended to be used by the"] # [doc = " release process to avoid cloning the monorepo and building stuff."] # [doc = ""] # [doc = " Should not be considered stable by end users."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct BuildManifest { target : TargetSelection , }
};
}
