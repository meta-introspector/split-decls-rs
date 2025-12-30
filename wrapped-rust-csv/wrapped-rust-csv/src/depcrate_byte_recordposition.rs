// Generated macro for Position (struct)
macro_rules! Depcrate_byte_recordPosition {
() => {
// Module: crate::byte_record
// Provides: {"Position"}
// Dependencies: {}
# [doc = " A position in CSV data."] # [doc = ""] # [doc = " A position is used to report errors in CSV data. All positions include the"] # [doc = " byte offset, line number and record index at which the error occurred."] # [doc = ""] # [doc = " Byte offsets and record indices start at `0`. Line numbers start at `1`."] # [doc = ""] # [doc = " A CSV reader will automatically assign the position of each record."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Position { byte : u64 , line : u64 , record : u64 , }
};
}
