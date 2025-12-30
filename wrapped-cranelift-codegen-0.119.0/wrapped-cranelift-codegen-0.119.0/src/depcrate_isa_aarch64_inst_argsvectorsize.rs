// Generated macro for VectorSize (enum)
macro_rules! Depcrate_isa_aarch64_inst_argsVectorSize {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"VectorSize"}
// Dependencies: {}
# [doc = " Type used to communicate the size of a vector operand."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum VectorSize { # [doc = " 8-bit, 8 lanes."] Size8x8 , # [doc = " 8 bit, 16 lanes."] Size8x16 , # [doc = " 16-bit, 4 lanes."] Size16x4 , # [doc = " 16-bit, 8 lanes."] Size16x8 , # [doc = " 32-bit, 2 lanes."] Size32x2 , # [doc = " 32-bit, 4 lanes."] Size32x4 , # [doc = " 64-bit, 2 lanes."] Size64x2 , }
};
}
