// Generated macro for Writer (struct)
macro_rules! Depcrate_writerWriter {
() => {
// Module: crate::writer
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " A writer for CSV data."] # [doc = ""] # [doc = " # RFC 4180"] # [doc = ""] # [doc = " This writer conforms to RFC 4180 with one exception: it doesn't guarantee"] # [doc = " that all records written are of the same length. Instead, the onus is on"] # [doc = " the caller to ensure that all records written are of the same length."] # [doc = ""] # [doc = " Note that the default configuration of a `Writer` uses `\\n` for record"] # [doc = " terminators instead of `\\r\\n` as specified by RFC 4180. Use the"] # [doc = " `terminator` method on `WriterBuilder` to set the terminator to `\\r\\n` if"] # [doc = " it's desired."] pub struct Writer { state : WriterState , requires_quotes : [bool ; 256] , delimiter : u8 , term : Terminator , style : QuoteStyle , quote : u8 , escape : u8 , double_quote : bool , comment : Option < u8 > , }
};
}
