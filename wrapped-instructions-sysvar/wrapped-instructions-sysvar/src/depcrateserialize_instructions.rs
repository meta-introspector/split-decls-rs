// Generated macro for serialize_instructions (function)
macro_rules! Depcrateserialize_instructions {
() => {
// Module: crate
// Provides: {"serialize_instructions"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] # [cfg_attr (feature = "dev-context-only-utils" , qualifiers (pub))] fn serialize_instructions (instructions : & [BorrowedInstruction]) -> Vec < u8 > { let mut data = Vec :: with_capacity (instructions . len () * (32 * 2)) ; append_u16 (& mut data , instructions . len () as u16) ; for _ in 0 .. instructions . len () { append_u16 (& mut data , 0) ; } for (i , instruction) in instructions . iter () . enumerate () { let start_instruction_offset = data . len () as u16 ; let start = 2 + (2 * i) ; data [start .. start + 2] . copy_from_slice (& start_instruction_offset . to_le_bytes ()) ; append_u16 (& mut data , instruction . accounts . len () as u16) ; for account_meta in & instruction . accounts { let mut account_meta_flags = InstructionsSysvarAccountMeta :: empty () ; if account_meta . is_signer { account_meta_flags |= InstructionsSysvarAccountMeta :: IS_SIGNER ; } if account_meta . is_writable { account_meta_flags |= InstructionsSysvarAccountMeta :: IS_WRITABLE ; } append_u8 (& mut data , account_meta_flags . bits ()) ; append_slice (& mut data , account_meta . pubkey . as_ref ()) ; } append_slice (& mut data , instruction . program_id . as_ref ()) ; append_u16 (& mut data , instruction . data . len () as u16) ; append_slice (& mut data , instruction . data) ; } data }
};
}
