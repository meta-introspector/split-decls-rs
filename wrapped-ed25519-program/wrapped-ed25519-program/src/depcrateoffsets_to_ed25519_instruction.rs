// Generated macro for offsets_to_ed25519_instruction (function)
macro_rules! Depcrateoffsets_to_ed25519_instruction {
() => {
// Module: crate
// Provides: {"offsets_to_ed25519_instruction"}
// Dependencies: {}
# [doc = " Encode just the signature offsets in a single ed25519 instruction."] # [doc = ""] # [doc = " This is a convenience function for rare cases where we wish to verify multiple messages in"] # [doc = " the same instruction. The verification data can be stored in a separate instruction specified"] # [doc = " by the `*_instruction_index` fields of `offsets`, or in this instruction by extending the data"] # [doc = " buffer."] # [doc = ""] # [doc = " Note: If the signer for these messages are the same, it is cheaper to concatenate the messages"] # [doc = " and have the signer sign the single buffer and use [`new_ed25519_instruction_with_signature`]."] pub fn offsets_to_ed25519_instruction (offsets : & [Ed25519SignatureOffsets]) -> Instruction { let mut instruction_data = Vec :: with_capacity (SIGNATURE_OFFSETS_START . saturating_add (SIGNATURE_OFFSETS_SERIALIZED_SIZE . saturating_mul (offsets . len ())) ,) ; let num_signatures = offsets . len () as u16 ; instruction_data . extend_from_slice (& num_signatures . to_le_bytes ()) ; for offsets in offsets { instruction_data . extend_from_slice (bytes_of (offsets)) ; } Instruction { program_id : solana_sdk_ids :: ed25519_program :: id () , accounts : vec ! [] , data : instruction_data , } }
};
}
