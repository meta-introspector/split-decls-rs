// Generated macro for Shared (struct)
macro_rules! Depcrate_mpscShared {
() => {
// Module: crate::mpsc
// Provides: {"Shared"}
// Dependencies: {}
# [derive (Debug)] struct Shared < T > { buffer : VecDeque < T > , blocked_recv : LocalWaker , has_receiver : bool , }
};
}
