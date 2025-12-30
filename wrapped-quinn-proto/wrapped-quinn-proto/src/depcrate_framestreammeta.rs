// Generated macro for StreamMeta (struct)
macro_rules! Depcrate_frameStreamMeta {
() => {
// Module: crate::frame
// Provides: {"StreamMeta"}
// Dependencies: {}
# [doc = " Metadata from a stream frame"] # [derive (Debug , Clone)] pub (crate) struct StreamMeta { pub (crate) id : StreamId , pub (crate) offsets : Range < u64 > , pub (crate) fin : bool , }
};
}
