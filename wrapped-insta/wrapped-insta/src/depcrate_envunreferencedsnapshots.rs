// Generated macro for UnreferencedSnapshots (enum)
macro_rules! Depcrate_envUnreferencedSnapshots {
() => {
// Module: crate::env
// Provides: {"UnreferencedSnapshots"}
// Dependencies: {}
# [doc = " Unreferenced snapshots flag"] # [cfg (feature = "_cargo_insta_internal")] # [derive (Clone , Copy , Debug , PartialEq , Eq , clap :: ValueEnum)] pub enum UnreferencedSnapshots { Auto , Reject , Delete , Warn , Ignore , }
};
}
