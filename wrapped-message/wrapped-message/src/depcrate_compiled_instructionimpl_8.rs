// Generated macro for impl_8 (impl)
macro_rules! Depcrate_compiled_instructionimpl_8 {
() => {
// Module: crate::compiled_instruction
// Provides: {"impl_8"}
// Dependencies: {}
impl CompiledInstruction { # [cfg (feature = "bincode")] pub fn new < T : serde :: Serialize > (program_ids_index : u8 , data : & T , accounts : Vec < u8 >) -> Self { let data = bincode :: serialize (data) . unwrap () ; Self { program_id_index : program_ids_index , accounts , data , } } pub fn new_from_raw_parts (program_id_index : u8 , data : Vec < u8 > , accounts : Vec < u8 >) -> Self { Self { program_id_index , accounts , data , } } pub fn program_id < 'a > (& self , program_ids : & 'a [Address]) -> & 'a Address { & program_ids [self . program_id_index as usize] } }
};
}
