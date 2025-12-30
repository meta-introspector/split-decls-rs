// Generated macro for SessionKeys (struct)
macro_rules! Depcrate_high_level_kexSessionKeys {
() => {
// Module: crate::high_level::kex
// Provides: {"SessionKeys"}
// Dependencies: {}
# [allow (clippy :: derive_partial_eq_without_eq)] # [derive (Debug , PartialEq)] # [doc = " A set of shared secrets for either transmitting to this entity or send to another party."] pub struct SessionKeys { rx : SecretKey , tx : SecretKey , }
};
}
