// Generated macro for PasswordVerifier (trait)
macro_rules! Depcrate_traitsPasswordVerifier {
() => {
// Module: crate::traits
// Provides: {"PasswordVerifier"}
// Dependencies: {}
# [doc = " Trait for password verification."] # [doc = ""] # [doc = " Automatically impl'd for any type that impls [`PasswordHasher`]."] # [doc = ""] # [doc = " This trait is object safe and can be used to implement abstractions over"] # [doc = " multiple password hashing algorithms. One such abstraction is provided by"] # [doc = " the [`PasswordHash::verify_password`] method."] pub trait PasswordVerifier { # [doc = " Compute this password hashing function against the provided password"] # [doc = " using the parameters from the provided password hash and see if the"] # [doc = " computed output matches."] fn verify_password (& self , password : & [u8] , hash : & PasswordHash < '_ >) -> Result < () > ; }
};
}
