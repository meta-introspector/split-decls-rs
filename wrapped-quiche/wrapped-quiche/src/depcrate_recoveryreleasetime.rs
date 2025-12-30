// Generated macro for ReleaseTime (enum)
macro_rules! Depcrate_recoveryReleaseTime {
() => {
// Module: crate::recovery
// Provides: {"ReleaseTime"}
// Dependencies: {}
# [doc = " When the pacer thinks is a good time to release the next packet"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum ReleaseTime { Immediate , At (Instant) , }
};
}
