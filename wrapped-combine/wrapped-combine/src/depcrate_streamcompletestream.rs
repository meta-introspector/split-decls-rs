// Generated macro for CompleteStream (struct)
macro_rules! Depcrate_streamCompleteStream {
() => {
// Module: crate::stream
// Provides: {"CompleteStream"}
// Dependencies: {}
# [doc = " Stream type which indicates that the stream is complete if end of input is reached"] # [doc = ""] # [doc = " For most streams this is already the default but this wrapper can be used to override a nested"] # [doc = " `PartialStream`"] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] # [repr (transparent)] pub struct CompleteStream < S > (pub S) ;
};
}
