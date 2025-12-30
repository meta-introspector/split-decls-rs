// Generated macro for Token (enum)
macro_rules! Depcrate_registryToken {
() => {
// Module: crate::registry
// Provides: {"Token"}
// Dependencies: {}
# [doc = " Auth-token for publishing, see [`RegistryBuilder::token`]"] # [derive (Clone)] pub enum Token { Plaintext (String) , Keys (String , Option < String >) , }
};
}
