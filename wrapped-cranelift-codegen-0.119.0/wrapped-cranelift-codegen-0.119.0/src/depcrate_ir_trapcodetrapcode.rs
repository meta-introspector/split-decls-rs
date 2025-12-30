// Generated macro for TrapCode (struct)
macro_rules! Depcrate_ir_trapcodeTrapCode {
() => {
// Module: crate::ir::trapcode
// Provides: {"TrapCode"}
// Dependencies: {}
# [doc = " A trap code describing the reason for a trap."] # [doc = ""] # [doc = " All trap instructions have an explicit trap code."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct TrapCode (NonZeroU8) ;
};
}
