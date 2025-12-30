// Generated macro for VersionVec (struct)
macro_rules! Depcrate_rt_vvVersionVec {
() => {
// Module: crate::rt::vv
// Provides: {"VersionVec"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Eq , PartialEq)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (crate) struct VersionVec { versions : [u16 ; MAX_THREADS] , }
};
}
