// Generated macro for ValueData (enum)
macro_rules! Depcrate_ir_dfgValueData {
() => {
// Module: crate::ir::dfg
// Provides: {"ValueData"}
// Dependencies: {}
# [doc = " Internal table storage for extended values."] # [derive (Clone , Debug , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] enum ValueData { # [doc = " Value is defined by an instruction."] Inst { ty : Type , num : u16 , inst : Inst } , # [doc = " Value is a block parameter."] Param { ty : Type , num : u16 , block : Block } , # [doc = " Value is an alias of another value."] # [doc = " An alias value can't be linked as an instruction result or block parameter. It is used as a"] # [doc = " placeholder when the original instruction or block has been rewritten or modified."] Alias { ty : Type , original : Value } , # [doc = " Union is a \"fork\" in representation: the value can be"] # [doc = " represented as either of the values named here. This is used"] # [doc = " for aegraph (acyclic egraph) representation in the DFG."] Union { ty : Type , x : Value , y : Value } , }
};
}
