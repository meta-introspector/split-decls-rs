// Generated macro for impl_408 (impl)
macro_rules! Depcrate_envelopeimpl_408 {
() => {
// Module: crate::envelope
// Provides: {"impl_408"}
// Dependencies: {}
impl Open { # [doc = " Creates a new `Open`."] pub fn new < T > (cipher : Cipher , priv_key : & PKeyRef < T > , iv : Option < & [u8] > , encrypted_key : & [u8] ,) -> Result < Open , ErrorStack > where T : HasPrivate , { let mut ctx = CipherCtx :: new () ? ; ctx . open_init (Some (unsafe { CipherRef :: from_ptr (cipher . as_ptr () as * mut _) }) , encrypted_key , iv , Some (priv_key) ,) ? ; Ok (Open { ctx }) } # [doc = " Feeds data from `input` through the cipher, writing decrypted bytes into `output`."] # [doc = ""] # [doc = " The number of bytes written to `output` is returned. Note that this may"] # [doc = " not be equal to the length of `input`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `output.len() < input.len() + block_size` where"] # [doc = " `block_size` is the block size of the cipher (see `Cipher::block_size`),"] # [doc = " or if `output.len() > c_int::MAX`."] pub fn update (& mut self , input : & [u8] , output : & mut [u8]) -> Result < usize , ErrorStack > { self . ctx . cipher_update (input , Some (output)) } # [doc = " Finishes the decryption process, writing any remaining data to `output`."] # [doc = ""] # [doc = " The number of bytes written to `output` is returned."] # [doc = ""] # [doc = " `update` should not be called after this method."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `output` is less than the cipher's block size."] pub fn finalize (& mut self , output : & mut [u8]) -> Result < usize , ErrorStack > { self . ctx . cipher_final (output) } }
};
}
