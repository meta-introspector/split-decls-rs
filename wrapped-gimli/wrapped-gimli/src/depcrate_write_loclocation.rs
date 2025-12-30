// Generated macro for Location (enum)
macro_rules! Depcrate_write_locLocation {
() => {
// Module: crate::write::loc
// Provides: {"Location"}
// Dependencies: {}
# [doc = " A single location."] # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub enum Location { # [doc = " DW_LLE_base_address"] BaseAddress { # [doc = " Base address."] address : Address , } , # [doc = " DW_LLE_offset_pair"] OffsetPair { # [doc = " Start of range relative to base address."] begin : u64 , # [doc = " End of range relative to base address."] end : u64 , # [doc = " Location description."] data : Expression , } , # [doc = " DW_LLE_start_end"] StartEnd { # [doc = " Start of range."] begin : Address , # [doc = " End of range."] end : Address , # [doc = " Location description."] data : Expression , } , # [doc = " DW_LLE_start_length"] StartLength { # [doc = " Start of range."] begin : Address , # [doc = " Length of range."] length : u64 , # [doc = " Location description."] data : Expression , } , # [doc = " DW_LLE_default_location"] DefaultLocation { # [doc = " Location description."] data : Expression , } , }
};
}
