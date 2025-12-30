// Generated macro for Headers (struct)
macro_rules! Depcrate_readerHeaders {
() => {
// Module: crate::reader
// Provides: {"Headers"}
// Dependencies: {}
# [doc = " Headers encapsulates any data associated with the headers of CSV data."] # [doc = ""] # [doc = " The headers always correspond to the first row."] # [derive (Debug)] struct Headers { # [doc = " The header, as raw bytes."] byte_record : ByteRecord , # [doc = " The header, as valid UTF-8 (or a UTF-8 error)."] string_record : result :: Result < StringRecord , Utf8Error > , }
};
}
