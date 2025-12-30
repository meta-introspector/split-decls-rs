// Generated macro for Role (enum)
macro_rules! Depcrate_hazardous_hpkeRole {
() => {
// Module: crate::hazardous::hpke
// Provides: {"Role"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] # [doc = " The role for an instance of HPKE mode."] pub enum Role { # [doc = " HPKE instance for encrypting data."] Sender , # [doc = " HPKE instance for decrypting data."] Recipient , }
};
}
