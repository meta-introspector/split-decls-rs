// Generated macro for InnerInstruction (struct)
macro_rules! Depcrate_inner_instructionInnerInstruction {
() => {
// Module: crate::inner_instruction
// Provides: {"InnerInstruction"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize) , serde (rename_all = "camelCase"))] pub struct InnerInstruction { pub instruction : CompiledInstruction , # [doc = " Invocation stack height of this instruction. Instruction stack height"] # [doc = " starts at 1 for transaction instructions."] pub stack_height : u8 , }
};
}
