// Generated macro for Encapsulate (trait)
macro_rules! DepcrateEncapsulate {
() => {
// Module: crate
// Provides: {"Encapsulate"}
// Dependencies: {}
# [doc = " A value that can be encapsulated to. Often, this will just be a public key. However, it can"] # [doc = " also be a bundle of public keys, or it can include a sender's private key for authenticated"] # [doc = " encapsulation."] pub trait Encapsulate < EK , SS > { # [doc = " Encapsulation error"] type Error : Debug ; # [doc = " Encapsulates a fresh shared secret"] fn encapsulate < R : TryCryptoRng + ? Sized > (& self , rng : & mut R) -> Result < (EK , SS) , Self :: Error > ; }
};
}
