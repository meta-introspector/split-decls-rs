// Generated macro for ReadFieldNoCopyResult (enum)
macro_rules! Depcrate_readerReadFieldNoCopyResult {
() => {
// Module: crate::reader
// Provides: {"ReadFieldNoCopyResult"}
// Dependencies: {}
# [doc = " The result of parsing at most one field from CSV data while ignoring the"] # [doc = " output."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ReadFieldNoCopyResult { # [doc = " The caller provided input was exhausted before the end of a field or"] # [doc = " record was found."] InputEmpty , # [doc = " The end of a field was found."] # [doc = ""] # [doc = " Note that when `record_end` is true, then the end of this field also"] # [doc = " corresponds to the end of a record."] Field { # [doc = " Whether this was the last field in a record or not."] record_end : bool , } , # [doc = " All CSV data has been read."] # [doc = ""] # [doc = " This state can only be returned when an empty input buffer is provided"] # [doc = " by the caller."] End , }
};
}
