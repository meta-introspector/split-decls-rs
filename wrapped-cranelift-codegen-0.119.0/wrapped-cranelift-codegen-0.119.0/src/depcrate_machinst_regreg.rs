// Generated macro for Reg (struct)
macro_rules! Depcrate_machinst_regReg {
() => {
// Module: crate::machinst::reg
// Provides: {"Reg"}
// Dependencies: {}
# [doc = " A register named in an instruction. This register can be either a"] # [doc = " virtual register or a fixed physical register. It does not have"] # [doc = " any constraints applied to it: those can be added later in"] # [doc = " `MachInst::get_operands()` when the `Reg`s are converted to"] # [doc = " `Operand`s."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Reg (VReg) ;
};
}
