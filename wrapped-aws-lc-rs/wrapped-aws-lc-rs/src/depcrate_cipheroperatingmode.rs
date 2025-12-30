// Generated macro for OperatingMode (enum)
macro_rules! Depcrate_cipherOperatingMode {
() => {
// Module: crate::cipher
// Provides: {"OperatingMode"}
// Dependencies: {}
# [doc = " The cipher operating mode."] # [non_exhaustive] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum OperatingMode { # [doc = " Cipher block chaining (CBC) mode."] CBC , # [doc = " Counter (CTR) mode."] CTR , # [doc = " CFB 128-bit mode."] CFB128 , # [doc = " Electronic Code Book (ECB) mode."] ECB , }
};
}
