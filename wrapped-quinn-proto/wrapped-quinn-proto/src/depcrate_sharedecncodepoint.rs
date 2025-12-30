// Generated macro for EcnCodepoint (enum)
macro_rules! Depcrate_sharedEcnCodepoint {
() => {
// Module: crate::shared
// Provides: {"EcnCodepoint"}
// Dependencies: {}
# [doc = " Explicit congestion notification codepoint"] # [repr (u8)] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum EcnCodepoint { # [doc = " The ECT(0) codepoint, indicating that an endpoint is ECN-capable"] Ect0 = 0b10 , # [doc = " The ECT(1) codepoint, indicating that an endpoint is ECN-capable"] Ect1 = 0b01 , # [doc = " The CE codepoint, signalling that congestion was experienced"] Ce = 0b11 , }
};
}
