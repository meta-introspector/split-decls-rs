// Generated macro for Offset32 (struct)
macro_rules! Depcrate_ir_immediatesOffset32 {
() => {
// Module: crate::ir::immediates
// Provides: {"Offset32"}
// Dependencies: {}
# [doc = " 32-bit signed immediate offset."] # [doc = ""] # [doc = " This is used to encode an immediate offset for load/store instructions. All supported ISAs have"] # [doc = " a maximum load/store offset that fits in an `i32`."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Offset32 (i32) ;
};
}
