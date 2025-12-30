// Generated macro for Decapsulate (trait)
macro_rules! DepcrateDecapsulate {
() => {
// Module: crate
// Provides: {"Decapsulate"}
// Dependencies: {}
# [doc = " A value that can be used to decapsulate an encapsulated key."] # [doc = ""] # [doc = " Often, this will just be a secret key. But, as with [`Encapsulate`], it can be a bundle"] # [doc = " of secret keys, or it can include a sender's private key for authenticated encapsulation."] pub trait Decapsulate < EK , SS > { # [doc = " Encapsulator which corresponds to this decapsulator."] type Encapsulator : Encapsulate < EK , SS > ; # [doc = " Decapsulation error"] type Error : Debug ; # [doc = " Decapsulates the given encapsulated key"] fn decapsulate (& self , encapsulated_key : & EK) -> Result < SS , Self :: Error > ; # [doc = " Retrieve the encapsulator associated with this decapsulator."] fn encapsulator (& self) -> Self :: Encapsulator ; }
};
}
