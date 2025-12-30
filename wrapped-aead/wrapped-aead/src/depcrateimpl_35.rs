// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
# [allow (deprecated)] impl < T : AeadInOut > AeadInPlace for T { fn encrypt_in_place (& self , nonce : & Nonce < Self > , associated_data : & [u8] , buffer : & mut dyn Buffer ,) -> Result < () > { < Self as AeadInOut > :: encrypt_in_place (self , nonce , associated_data , buffer) } fn encrypt_in_place_detached (& self , nonce : & Nonce < Self > , associated_data : & [u8] , buffer : & mut [u8] ,) -> Result < Tag < Self > > { self . encrypt_inout_detached (nonce , associated_data , buffer . into ()) } fn decrypt_in_place (& self , nonce : & Nonce < Self > , associated_data : & [u8] , buffer : & mut dyn Buffer ,) -> Result < () > { < Self as AeadInOut > :: decrypt_in_place (self , nonce , associated_data , buffer) } fn decrypt_in_place_detached (& self , nonce : & Nonce < Self > , associated_data : & [u8] , buffer : & mut [u8] , tag : & Tag < Self > ,) -> Result < () > { self . decrypt_inout_detached (nonce , associated_data , buffer . into () , tag) } }
};
}
