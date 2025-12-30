// Generated macro for AtomicRmwOp (enum)
macro_rules! Depcrate_ir_atomic_rmw_opAtomicRmwOp {
() => {
// Module: crate::ir::atomic_rmw_op
// Provides: {"AtomicRmwOp"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] # [doc = " Describes the arithmetic operation in an atomic memory read-modify-write operation."] pub enum AtomicRmwOp { # [doc = " Add"] Add , # [doc = " Sub"] Sub , # [doc = " And"] And , # [doc = " Nand"] Nand , # [doc = " Or"] Or , # [doc = " Xor"] Xor , # [doc = " Exchange"] Xchg , # [doc = " Unsigned min"] Umin , # [doc = " Unsigned max"] Umax , # [doc = " Signed min"] Smin , # [doc = " Signed max"] Smax , }
};
}
