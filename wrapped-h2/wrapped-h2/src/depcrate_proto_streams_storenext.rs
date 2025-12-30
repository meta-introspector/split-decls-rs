// Generated macro for Next (trait)
macro_rules! Depcrate_proto_streams_storeNext {
() => {
// Module: crate::proto::streams::store
// Provides: {"Next"}
// Dependencies: {}
pub (super) trait Next { fn next (stream : & Stream) -> Option < Key > ; fn set_next (stream : & mut Stream , key : Option < Key >) ; fn take_next (stream : & mut Stream) -> Option < Key > ; fn is_queued (stream : & Stream) -> bool ; fn set_queued (stream : & mut Stream , val : bool) ; }
};
}
