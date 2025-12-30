// Generated macro for impl_330 (impl)
macro_rules! Depcrate_key_wrapimpl_330 {
() => {
// Module: crate::key_wrap
// Provides: {"impl_330"}
// Dependencies: {}
impl < Cipher : BlockCipher > KeyEncryptionKey < Cipher > { # [doc = " Construct a new Key Encryption Key."] # [doc = ""] # [doc = " # Errors"] # [doc = " * [`Unspecified`]: Any error that occurs constructing the key encryption key."] pub fn new (cipher : & 'static Cipher , key : & [u8]) -> Result < Self , Unspecified > { if key . len () != cipher . key_len () { return Err (Unspecified) ; } let key = Vec :: from (key) . into_boxed_slice () ; Ok (Self { cipher , key }) } # [doc = " Returns the block cipher algorithm identifier configured for the key."] # [must_use] pub fn block_cipher_id (& self) -> BlockCipherId { self . cipher . id () } }
};
}
