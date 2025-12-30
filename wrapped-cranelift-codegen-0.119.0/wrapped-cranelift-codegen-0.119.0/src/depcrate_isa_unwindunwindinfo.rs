// Generated macro for UnwindInfo (enum)
macro_rules! Depcrate_isa_unwindUnwindInfo {
() => {
// Module: crate::isa::unwind
// Provides: {"UnwindInfo"}
// Dependencies: {}
# [doc = " Represents unwind information for a single function."] # [derive (Clone , Debug , PartialEq , Eq)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] # [non_exhaustive] pub enum UnwindInfo { # [doc = " Windows x64 ABI unwind information."] # [cfg (feature = "unwind")] WindowsX64 (winx64 :: UnwindInfo) , # [doc = " System V ABI unwind information."] # [cfg (feature = "unwind")] SystemV (CfaUnwindInfo) , # [doc = " Windows Arm64 ABI unwind information."] # [cfg (feature = "unwind")] WindowsArm64 (winarm64 :: UnwindInfo) , }
};
}
