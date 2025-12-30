// Generated macro for TeeBuffer (struct)
macro_rules! Depcrate_teeTeeBuffer {
() => {
// Module: crate::tee
// Provides: {"TeeBuffer"}
// Dependencies: {}
# [doc = " Common buffer object for the two tee halves"] # [derive (Debug)] struct TeeBuffer < A , I > { backlog : VecDeque < A > , iter : I , # [doc = " The owner field indicates which id should read from the backlog"] owner : bool , }
};
}
