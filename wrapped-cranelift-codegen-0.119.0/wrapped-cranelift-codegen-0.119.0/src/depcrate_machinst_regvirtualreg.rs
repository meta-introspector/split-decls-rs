// Generated macro for VirtualReg (struct)
macro_rules! Depcrate_machinst_regVirtualReg {
() => {
// Module: crate::machinst::reg
// Provides: {"VirtualReg"}
// Dependencies: {}
# [doc = " A virtual register. This can be allocated into a real (physical)"] # [doc = " register of the appropriate register class, but which one is not"] # [doc = " specified. Virtual registers are used when generating `MachInst`s,"] # [doc = " before register allocation occurs, in order to allow us to name as"] # [doc = " many register-carried values as necessary."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct VirtualReg (VReg) ;
};
}
