// Generated macro for RetryError (struct)
macro_rules! Depcrate_incomingRetryError {
() => {
// Module: crate::incoming
// Provides: {"RetryError"}
// Dependencies: {}
# [doc = " Error for attempting to retry an [`Incoming`] which already bears a token from a previous retry"] # [derive (Debug , Error)] # [error ("retry() with validated Incoming")] pub struct RetryError (Box < Incoming >) ;
};
}
