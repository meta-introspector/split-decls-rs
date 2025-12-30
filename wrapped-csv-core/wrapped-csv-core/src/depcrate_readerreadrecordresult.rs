// Generated macro for ReadRecordResult (enum)
macro_rules! Depcrate_readerReadRecordResult {
() => {
// Module: crate::reader
// Provides: {"ReadRecordResult"}
// Dependencies: {}
# [doc = " The result of parsing at most one record from CSV data."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ReadRecordResult { # [doc = " The caller provided input was exhausted before the end of a record was"] # [doc = " found."] InputEmpty , # [doc = " The caller provided output buffer was filled before an entire field"] # [doc = " could be written to it."] OutputFull , # [doc = " The caller provided output buffer of field end poisitions was filled"] # [doc = " before the next field could be parsed."] OutputEndsFull , # [doc = " The end of a record was found."] Record , # [doc = " All CSV data has been read."] # [doc = ""] # [doc = " This state can only be returned when an empty input buffer is provided"] # [doc = " by the caller."] End , }
};
}
