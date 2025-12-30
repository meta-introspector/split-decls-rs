// Generated macro for Header (struct)
macro_rules! Depcrate_tz_concatenatedHeader {
() => {
// Module: crate::tz::concatenated
// Provides: {"Header"}
// Dependencies: {}
# [doc = " The header of Android concatenated TZif data."] # [doc = ""] # [doc = " The header has the version and some offsets indicating the location of"] # [doc = " the index entry (a list of IANA time zone identifiers and offsets into"] # [doc = " the data block) and the actual TZif data."] # [derive (Debug)] struct Header { version : ArrayStr < 5 > , index_offset : u64 , data_offset : u64 , }
};
}
