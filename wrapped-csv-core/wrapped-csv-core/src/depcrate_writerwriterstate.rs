// Generated macro for WriterState (struct)
macro_rules! Depcrate_writerWriterState {
() => {
// Module: crate::writer
// Provides: {"WriterState"}
// Dependencies: {}
# [derive (Clone , Debug)] struct WriterState { # [doc = " This is set whenever we've begun writing the contents of a field, even"] # [doc = " if the contents are empty. We use it to avoid re-computing whether"] # [doc = " quotes are necessary."] in_field : bool , # [doc = " This is set whenever we've started writing a field that is enclosed in"] # [doc = " quotes. When the writer is finished, or if a delimiter or terminator"] # [doc = " are written, then a closing quote is inserted when this is true."] quoting : bool , # [doc = " The number of total bytes written for the current record."] # [doc = ""] # [doc = " If the writer is finished or a terminator is written when this is `0`,"] # [doc = " then an empty field is added as a pair of adjacent quotes."] record_bytes : u64 , }
};
}
