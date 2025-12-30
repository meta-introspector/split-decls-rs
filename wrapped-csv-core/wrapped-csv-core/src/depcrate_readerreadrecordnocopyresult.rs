// Generated macro for ReadRecordNoCopyResult (enum)
macro_rules! Depcrate_readerReadRecordNoCopyResult {
() => {
// Module: crate::reader
// Provides: {"ReadRecordNoCopyResult"}
// Dependencies: {}
# [doc = " The result of parsing at most one record from CSV data while ignoring"] # [doc = " output."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ReadRecordNoCopyResult { # [doc = " The caller provided input was exhausted before the end of a record was"] # [doc = " found."] InputEmpty , # [doc = " The end of a record was found."] Record , # [doc = " All CSV data has been read."] # [doc = ""] # [doc = " This state can only be returned when an empty input buffer is provided"] # [doc = " by the caller."] End , }
};
}
