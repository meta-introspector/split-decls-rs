// Generated macro for deserialize_instruction (function)
macro_rules! Depcratedeserialize_instruction {
() => {
// Module: crate
// Provides: {"deserialize_instruction"}
// Dependencies: {}
# [cfg_attr (feature = "dev-context-only-utils" , qualifiers (pub))] fn deserialize_instruction (index : usize , data : & [u8]) -> Result < Instruction , SanitizeError > { const IS_SIGNER_BIT : usize = 0 ; const IS_WRITABLE_BIT : usize = 1 ; let mut current = 0 ; let num_instructions = read_u16 (& mut current , data) ? ; if index >= num_instructions as usize { return Err (SanitizeError :: IndexOutOfBounds) ; } current += index * 2 ; let start = read_u16 (& mut current , data) ? ; current = start as usize ; let num_accounts = read_u16 (& mut current , data) ? ; let mut accounts = Vec :: with_capacity (num_accounts as usize) ; for _ in 0 .. num_accounts { let meta_byte = read_u8 (& mut current , data) ? ; let mut is_signer = false ; let mut is_writable = false ; if meta_byte & (1 << IS_SIGNER_BIT) != 0 { is_signer = true ; } if meta_byte & (1 << IS_WRITABLE_BIT) != 0 { is_writable = true ; } let pubkey = read_pubkey (& mut current , data) ? ; accounts . push (AccountMeta { pubkey , is_signer , is_writable , }) ; } let program_id = read_pubkey (& mut current , data) ? ; let data_len = read_u16 (& mut current , data) ? ; let data = read_slice (& mut current , data , data_len as usize) ? ; Ok (Instruction { program_id , accounts , data , }) }
};
}
