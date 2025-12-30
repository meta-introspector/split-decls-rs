// Generated macro for Progress (enum)
macro_rules! Depcrate_flycheckProgress {
() => {
// Module: crate::flycheck
// Provides: {"Progress"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum Progress { DidStart , DidCheckCrate (String) , DidFinish (io :: Result < () >) , DidCancel , DidFailToRestart (String) , }
};
}
