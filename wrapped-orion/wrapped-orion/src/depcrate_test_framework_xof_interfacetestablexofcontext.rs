// Generated macro for TestableXofContext (trait)
macro_rules! Depcrate_test_framework_xof_interfaceTestableXofContext {
() => {
// Module: crate::test_framework::xof_interface
// Provides: {"TestableXofContext"}
// Dependencies: {}
# [doc = " Trait to define default XOF contexts that can be tested."] # [doc = ""] # [doc = " Based on `TestableStreamingContext` but with some corrections"] # [doc = " towards a XOF."] pub trait TestableXofContext { # [doc = " Streaming context function to reset the internal state."] fn reset (& mut self) -> Result < () , UnknownCryptoError > ; # [doc = " Streaming context function to update the internal state."] fn absorb (& mut self , input : & [u8]) -> Result < () , UnknownCryptoError > ; # [doc = " Streaming context function to finalize the internal state."] fn squeeze (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > ; # [doc = " Testing utility-function that compares the internal state to another."] fn compare_states (state_1 : & Self , state_2 : & Self) ; }
};
}
