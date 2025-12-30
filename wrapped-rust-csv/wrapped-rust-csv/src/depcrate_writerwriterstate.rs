// Generated macro for WriterState (struct)
macro_rules! Depcrate_writerWriterState {
() => {
// Module: crate::writer
// Provides: {"WriterState"}
// Dependencies: {}
# [derive (Debug)] struct WriterState { # [doc = " Whether the Serde serializer should attempt to write a header row."] header : HeaderState , # [doc = " Whether inconsistent record lengths are allowed."] flexible : bool , # [doc = " The number of fields written in the first record. This is compared"] # [doc = " with `fields_written` on all subsequent records to check for"] # [doc = " inconsistent record lengths."] first_field_count : Option < u64 > , # [doc = " The number of fields written in this record. This is used to report"] # [doc = " errors for inconsistent record lengths if `flexible` is disabled."] fields_written : u64 , # [doc = " This is set immediately before flushing the buffer and then unset"] # [doc = " immediately after flushing the buffer. This avoids flushing the buffer"] # [doc = " twice if the inner writer panics."] panicked : bool , }
};
}
