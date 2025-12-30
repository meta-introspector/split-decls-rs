// Generated macro for ReplayFileStatus (enum)
macro_rules! Depcrate_test_runner_replayReplayFileStatus {
() => {
// Module: crate::test_runner::replay
// Provides: {"ReplayFileStatus"}
// Dependencies: {}
# [doc = " Result of loading a replay file."] # [derive (Clone , Debug)] pub (crate) enum ReplayFileStatus { # [doc = " The file is valid and represents a currently-in-progress test."] InProgress (Replay) , # [doc = " The file is valid, but indicates that all testing has completed."] Terminated (Replay) , # [doc = " The file is not parsable."] Corrupt , }
};
}
