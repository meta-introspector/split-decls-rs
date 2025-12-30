// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl < C > KeyInit for RetailMacCore < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone + KeyInit , < C as BlockSizeUser > :: BlockSize : Mul < U2 > , Prod < < C as BlockSizeUser > :: BlockSize , U2 > : ArraySize , { # [inline (always)] fn new (key : & Key < Self >) -> Self { Self :: new_from_slice (key . as_slice ()) . unwrap () } # [inline (always)] fn new_from_slice (key : & [u8]) -> Result < Self , InvalidLength > { let cipher = C :: new_from_slice (& key [.. key . len () / 2]) ? ; let cipher_prime = C :: new_from_slice (& key [key . len () / 2 ..]) ? ; Ok (Self { cipher , cipher_prime , state : Block :: < Self > :: default () , }) } }
};
}
