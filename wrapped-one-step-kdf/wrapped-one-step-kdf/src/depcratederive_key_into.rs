// Generated macro for derive_key_into (function)
macro_rules! Depcratederive_key_into {
() => {
// Module: crate
// Provides: {"derive_key_into"}
// Dependencies: {}
# [doc = " Derives `key` in-place from `secret` and `other_info`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use hex_literal::hex;"] # [doc = " use sha2::Sha256;"] # [doc = ""] # [doc = " let mut key = [0u8; 16];"] # [doc = " one_step_kdf::derive_key_into::<Sha256>(b\"secret\", b\"shared-info\", &mut key).unwrap();"] # [doc = " assert_eq!(key, hex!(\"960db2c549ab16d71a7b008e005c2bdc\"));"] # [doc = " ```"] pub fn derive_key_into < D > (secret : & [u8] , other_info : & [u8] , key : & mut [u8]) -> Result < () , Error > where D : Digest + FixedOutputReset , { if secret . is_empty () { return Err (Error :: NoSecret) ; } if key . is_empty () { return Err (Error :: NoOutput) ; } if (key . len () as u64) >= D :: OutputSize :: U64 * (u32 :: MAX as u64) { return Err (Error :: CounterOverflow) ; } let mut digest = D :: new () ; let mut counter : u32 = 1 ; for chunk in key . chunks_mut (D :: OutputSize :: USIZE) { Update :: update (& mut digest , & counter . to_be_bytes ()) ; Update :: update (& mut digest , secret) ; Update :: update (& mut digest , other_info) ; chunk . copy_from_slice (& digest . finalize_reset () [.. chunk . len ()]) ; counter += 1 ; } Ok (()) }
};
}
