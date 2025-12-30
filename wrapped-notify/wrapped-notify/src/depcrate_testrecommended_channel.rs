// Generated macro for recommended_channel (function)
macro_rules! Depcrate_testrecommended_channel {
() => {
// Module: crate::test
// Provides: {"recommended_channel"}
// Dependencies: {}
# [doc = " Creates a [`TestWatcher`] for the [`RecommendedWatcher`] and connected [`Receiver`]"] pub fn recommended_channel () -> (TestWatcher < RecommendedWatcher > , Receiver) { channel () }
};
}
