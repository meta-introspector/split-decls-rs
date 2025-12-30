// Generated macro for ReleaseDecision (struct)
macro_rules! Depcrate_recoveryReleaseDecision {
() => {
// Module: crate::recovery
// Provides: {"ReleaseDecision"}
// Dependencies: {}
# [doc = " When the next packet should be release and if it can be part of a burst"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct ReleaseDecision { time : ReleaseTime , allow_burst : bool , }
};
}
