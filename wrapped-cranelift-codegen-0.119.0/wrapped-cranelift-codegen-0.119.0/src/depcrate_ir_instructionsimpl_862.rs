// Generated macro for impl_862 (impl)
macro_rules! Depcrate_ir_instructionsimpl_862 {
() => {
// Module: crate::ir::instructions
// Provides: {"impl_862"}
// Dependencies: {}
impl FromStr for Opcode { type Err = & 'static str ; # [doc = " Parse an Opcode name from a string."] fn from_str (s : & str) -> Result < Self , & 'static str > { use crate :: constant_hash :: { probe , simple_hash } ; match probe :: < & str , [Option < Self >] > (& OPCODE_HASH_TABLE , s , simple_hash (s)) { Err (_) => Err ("Unknown opcode") , Ok (i) => Ok (OPCODE_HASH_TABLE [i] . unwrap ()) , } } }
};
}
