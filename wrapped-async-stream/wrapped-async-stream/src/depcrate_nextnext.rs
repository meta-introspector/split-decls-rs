// Generated macro for next (function)
macro_rules! Depcrate_nextnext {
() => {
// Module: crate::next
// Provides: {"next"}
// Dependencies: {}
# [doc (hidden)] pub fn next < S > (stream : & mut S) -> impl Future < Output = Option < S :: Item > > + '_ where S : Stream + Unpin , { Next { stream } }
};
}
