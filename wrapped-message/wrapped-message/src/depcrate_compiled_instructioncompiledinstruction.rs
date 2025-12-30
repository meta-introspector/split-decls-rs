// Generated macro for CompiledInstruction (struct)
macro_rules! Depcrate_compiled_instructionCompiledInstruction {
() => {
// Module: crate::compiled_instruction
// Provides: {"CompiledInstruction"}
// Dependencies: {}
# [doc = " A compact encoding of an instruction."] # [doc = ""] # [doc = " A `CompiledInstruction` is a component of a multi-instruction [`Message`],"] # [doc = " which is the core of a Solana transaction. It is created during the"] # [doc = " construction of `Message`. Most users will not interact with it directly."] # [doc = ""] # [doc = " [`Message`]: crate::Message"] # [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize) , serde (rename_all = "camelCase"))] # [cfg_attr (feature = "wincode" , derive (SchemaWrite , SchemaRead))] # [derive (Debug , PartialEq , Eq , Clone)] pub struct CompiledInstruction { # [doc = " Index into the transaction keys array indicating the program account that executes this instruction."] pub program_id_index : u8 , # [doc = " Ordered indices into the transaction keys array indicating which accounts to pass to the program."] # [cfg_attr (feature = "serde" , serde (with = "solana_short_vec"))] # [cfg_attr (feature = "wincode" , wincode (with = "containers::Vec<_, ShortU16Len>"))] pub accounts : Vec < u8 > , # [doc = " The program input data."] # [cfg_attr (feature = "serde" , serde (with = "solana_short_vec"))] # [cfg_attr (feature = "wincode" , wincode (with = "containers::Vec<_, ShortU16Len>"))] pub data : Vec < u8 > , }
};
}
