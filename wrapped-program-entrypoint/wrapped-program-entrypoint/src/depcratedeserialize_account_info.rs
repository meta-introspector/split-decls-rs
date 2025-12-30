// Generated macro for deserialize_account_info (function)
macro_rules! Depcratedeserialize_account_info {
() => {
// Module: crate
// Provides: {"deserialize_account_info"}
// Dependencies: {}
# [allow (clippy :: arithmetic_side_effects)] # [inline (always)] unsafe fn deserialize_account_info < 'a > (input : * mut u8 , mut offset : usize ,) -> (AccountInfo < 'a > , usize) { # [allow (clippy :: cast_ptr_alignment)] let is_signer = * (input . add (offset) as * const u8) != 0 ; offset += size_of :: < u8 > () ; # [allow (clippy :: cast_ptr_alignment)] let is_writable = * (input . add (offset) as * const u8) != 0 ; offset += size_of :: < u8 > () ; # [allow (clippy :: cast_ptr_alignment)] let executable = * (input . add (offset) as * const u8) != 0 ; offset += size_of :: < u8 > () ; let original_data_len_offset = offset ; offset += size_of :: < u32 > () ; let key : & Pubkey = & * (input . add (offset) as * const Pubkey) ; offset += size_of :: < Pubkey > () ; let owner : & Pubkey = & * (input . add (offset) as * const Pubkey) ; offset += size_of :: < Pubkey > () ; # [allow (clippy :: cast_ptr_alignment)] let lamports = & mut * (input . add (offset) as * mut u64) ; offset += size_of :: < u64 > () ; # [allow (clippy :: cast_ptr_alignment)] let data_len = * (input . add (offset) as * const u64) as usize ; offset += size_of :: < u64 > () ; * (input . add (original_data_len_offset) as * mut u32) = data_len as u32 ; let data = from_raw_parts_mut (input . add (offset) , data_len) ; offset += data_len + MAX_PERMITTED_DATA_INCREASE + size_of :: < u64 > () ; offset += (offset as * const u8) . align_offset (BPF_ALIGN_OF_U128) ; (AccountInfo :: new (key , is_signer , is_writable , lamports , data , owner , executable ,) , offset ,) }
};
}
