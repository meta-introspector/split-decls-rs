// Generated macro for derive_block (function)
macro_rules! Depcrate_pbkdf2derive_block {
() => {
// Module: crate::pbkdf2
// Provides: {"derive_block"}
// Dependencies: {}
fn derive_block (secret : & hmac :: Key , iterations : NonZeroU32 , salt : & [u8] , idx : u32 , out : & mut [u8] , cpu : cpu :: Features ,) -> Result < () , InputTooLongError > { let mut ctx = hmac :: Context :: with_key (secret) ; ctx . update (salt) ; ctx . update (& u32 :: to_be_bytes (idx)) ; let mut u = ctx . try_sign (cpu) ? ; let mut remaining : u32 = iterations . into () ; loop { bb :: xor_assign_at_start_bytes (& mut out [..] , u . as_ref ()) ; if remaining == 1 { break ; } remaining -= 1 ; u = secret . sign (u . as_ref () , cpu) ? } Ok (()) }
};
}
