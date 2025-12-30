// Generated macro for AnyEntity (enum)
macro_rules! Depcrate_ir_entitiesAnyEntity {
() => {
// Module: crate::ir::entities
// Provides: {"AnyEntity"}
// Dependencies: {}
# [doc = " An opaque reference to any of the entities defined in this module that can appear in CLIF IR."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum AnyEntity { # [doc = " The whole function."] Function , # [doc = " a basic block."] Block (Block) , # [doc = " An instruction."] Inst (Inst) , # [doc = " An SSA value."] Value (Value) , # [doc = " A stack slot."] StackSlot (StackSlot) , # [doc = " A dynamic stack slot."] DynamicStackSlot (DynamicStackSlot) , # [doc = " A dynamic type"] DynamicType (DynamicType) , # [doc = " A Global value."] GlobalValue (GlobalValue) , # [doc = " A memory type."] MemoryType (MemoryType) , # [doc = " A jump table."] JumpTable (JumpTable) , # [doc = " A constant."] Constant (Constant) , # [doc = " An external function."] FuncRef (FuncRef) , # [doc = " A function call signature."] SigRef (SigRef) , # [doc = " A function's stack limit"] StackLimit , }
};
}
