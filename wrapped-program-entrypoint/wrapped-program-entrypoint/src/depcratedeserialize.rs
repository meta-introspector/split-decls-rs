// Generated macro for deserialize (function)
macro_rules! Depcratedeserialize {
() => {
// Module: crate
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize the input arguments"] # [doc = ""] # [doc = " The integer arithmetic in this method is safe when called on a buffer that was"] # [doc = " serialized by runtime. Use with buffers serialized otherwise is unsupported and"] # [doc = " done at one's own risk."] # [doc = ""] # [doc = " # Safety"] # [allow (clippy :: arithmetic_side_effects)] pub unsafe fn deserialize < 'a > (input : * mut u8) -> (& 'a Pubkey , Vec < AccountInfo < 'a > > , & 'a [u8]) { let mut offset : usize = 0 ; # [allow (clippy :: cast_ptr_alignment)] let num_accounts = * (input . add (offset) as * const u64) as usize ; offset += size_of :: < u64 > () ; let mut accounts = Vec :: with_capacity (num_accounts) ; for _ in 0 .. num_accounts { let dup_info = * (input . add (offset) as * const u8) ; offset += size_of :: < u8 > () ; if dup_info == NON_DUP_MARKER { let (account_info , new_offset) = deserialize_account_info (input , offset) ; offset = new_offset ; accounts . push (account_info) ; } else { offset += 7 ; accounts . push (accounts [dup_info as usize] . clone ()) ; } } let (instruction_data , new_offset) = deserialize_instruction_data (input , offset) ; offset = new_offset ; let program_id : & Pubkey = & * (input . add (offset) as * const Pubkey) ; (program_id , accounts , instruction_data) }
};
}
