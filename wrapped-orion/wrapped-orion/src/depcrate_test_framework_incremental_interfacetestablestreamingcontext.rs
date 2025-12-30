// Generated macro for TestableStreamingContext (trait)
macro_rules! Depcrate_test_framework_incremental_interfaceTestableStreamingContext {
() => {
// Module: crate::test_framework::incremental_interface
// Provides: {"TestableStreamingContext"}
// Dependencies: {}
# [doc = " Trait to define default streaming contexts that can be tested."] pub trait TestableStreamingContext < T : PartialEq > { # [doc = " Streaming context function to reset the internal state."] fn reset (& mut self) -> Result < () , UnknownCryptoError > ; # [doc = " Streaming context function to update the internal state."] fn update (& mut self , input : & [u8]) -> Result < () , UnknownCryptoError > ; # [doc = " Streaming context function to finalize the internal state."] fn finalize (& mut self) -> Result < T , UnknownCryptoError > ; # [doc = " Streaming context function to combine new(), update() and finalize() from the internal state."] fn one_shot (input : & [u8]) -> Result < T , UnknownCryptoError > ; # [doc = " Streaming context function to verify pre-computed results."] fn verify_result (expected : & T , input : & [u8]) -> Result < () , UnknownCryptoError > ; # [doc = " Testing utility-function that compares the internal state to another."] fn compare_states (state_1 : & Self , state_2 : & Self) ; }
};
}
