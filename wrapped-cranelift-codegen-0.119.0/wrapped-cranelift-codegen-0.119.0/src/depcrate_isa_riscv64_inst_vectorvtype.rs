// Generated macro for VType (struct)
macro_rules! Depcrate_isa_riscv64_inst_vectorVType {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"VType"}
// Dependencies: {}
# [doc = " Vector Type (VType)"] # [doc = ""] # [doc = " vtype provides the default type used to interpret the contents of the vector register file."] # [derive (Clone , Copy , Debug , PartialEq)] pub struct VType { pub sew : VecElementWidth , pub lmul : VecLmul , pub tail_mode : VecTailMode , pub mask_mode : VecMaskMode , }
};
}
