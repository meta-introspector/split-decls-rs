// Generated macro for JumpTable (struct)
macro_rules! Depcrate_ir_entitiesJumpTable {
() => {
// Module: crate::ir::entities
// Provides: {"JumpTable"}
// Dependencies: {}
# [doc = " An opaque reference to a [jump table](https://en.wikipedia.org/wiki/Branch_table)."] # [doc = ""] # [doc = " `JumpTable`s are used for indirect branching and are specialized for dense,"] # [doc = " 0-based jump offsets. If you want a jump table which doesn't start at 0,"] # [doc = " or is not contiguous, consider using a [`Switch`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.Switch.html) instead."] # [doc = ""] # [doc = " `JumpTable` are used with [`br_table`](super::InstBuilder::br_table)."] # [doc = ""] # [doc = " `JumpTable`s can be created with"] # [doc = " [`create_jump_table`](https://docs.rs/cranelift-frontend/*/cranelift_frontend/struct.FunctionBuilder.html#method.create_jump_table)."] # [doc = ""] # [doc = " While the order is stable, it is arbitrary."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct JumpTable (u32) ;
};
}
