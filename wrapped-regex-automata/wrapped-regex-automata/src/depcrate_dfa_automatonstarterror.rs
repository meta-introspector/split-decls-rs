// Generated macro for StartError (enum)
macro_rules! Depcrate_dfa_automatonStartError {
() => {
// Module: crate::dfa::automaton
// Provides: {"StartError"}
// Dependencies: {}
# [doc = " An error that can occur when computing the start state for a search."] # [doc = ""] # [doc = " Computing a start state can fail for a few reasons, either based on"] # [doc = " incorrect configuration or even based on whether the look-behind byte"] # [doc = " triggers a quit state. Typically one does not need to handle this error"] # [doc = " if you're using [`Automaton::start_state_forward`] (or its reverse"] # [doc = " counterpart), as that routine automatically converts `StartError` to a"] # [doc = " [`MatchError`] for you."] # [doc = ""] # [doc = " This error may be returned by the [`Automaton::start_state`] routine."] # [doc = ""] # [doc = " This error implements the `std::error::Error` trait when the `std` feature"] # [doc = " is enabled."] # [doc = ""] # [doc = " This error is marked as non-exhaustive. New variants may be added in a"] # [doc = " semver compatible release."] # [non_exhaustive] # [derive (Clone , Debug)] pub enum StartError { # [doc = " An error that occurs when a starting configuration's look-behind byte"] # [doc = " is in this DFA's quit set."] Quit { # [doc = " The quit byte that was found."] byte : u8 , } , # [doc = " An error that occurs when the caller requests an anchored mode that"] # [doc = " isn't supported by the DFA."] UnsupportedAnchored { # [doc = " The anchored mode given that is unsupported."] mode : Anchored , } , }
};
}
