// Generated macro for RawRngListEntry (enum)
macro_rules! Depcrate_read_rnglistsRawRngListEntry {
() => {
// Module: crate::read::rnglists
// Provides: {"RawRngListEntry"}
// Dependencies: {}
# [doc = " A raw entry in .debug_rnglists"] # [derive (Clone , Debug)] pub enum RawRngListEntry < T > { # [doc = " A range from DWARF version <= 4."] AddressOrOffsetPair { # [doc = " Start of range. May be an address or an offset."] begin : u64 , # [doc = " End of range. May be an address or an offset."] end : u64 , } , # [doc = " DW_RLE_base_address"] BaseAddress { # [doc = " base address"] addr : u64 , } , # [doc = " DW_RLE_base_addressx"] BaseAddressx { # [doc = " base address"] addr : DebugAddrIndex < T > , } , # [doc = " DW_RLE_startx_endx"] StartxEndx { # [doc = " start of range"] begin : DebugAddrIndex < T > , # [doc = " end of range"] end : DebugAddrIndex < T > , } , # [doc = " DW_RLE_startx_length"] StartxLength { # [doc = " start of range"] begin : DebugAddrIndex < T > , # [doc = " length of range"] length : u64 , } , # [doc = " DW_RLE_offset_pair"] OffsetPair { # [doc = " start of range"] begin : u64 , # [doc = " end of range"] end : u64 , } , # [doc = " DW_RLE_start_end"] StartEnd { # [doc = " start of range"] begin : u64 , # [doc = " end of range"] end : u64 , } , # [doc = " DW_RLE_start_length"] StartLength { # [doc = " start of range"] begin : u64 , # [doc = " length of range"] length : u64 , } , }
};
}
