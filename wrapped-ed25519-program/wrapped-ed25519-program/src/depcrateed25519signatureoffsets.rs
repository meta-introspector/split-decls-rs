// Generated macro for Ed25519SignatureOffsets (struct)
macro_rules! DepcrateEd25519SignatureOffsets {
() => {
// Module: crate
// Provides: {"Ed25519SignatureOffsets"}
// Dependencies: {}
# [derive (Default , Debug , Copy , Clone , Zeroable , Pod , Eq , PartialEq)] # [repr (C)] pub struct Ed25519SignatureOffsets { pub signature_offset : u16 , pub signature_instruction_index : u16 , pub public_key_offset : u16 , pub public_key_instruction_index : u16 , pub message_data_offset : u16 , pub message_data_size : u16 , pub message_instruction_index : u16 , }
};
}
