macro_rules! deps {
    () => {
        Address!();
    };
}

macro_rules! Range {
    () => {
        deps!();
        # [doc = " A single range."] # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub enum Range { # [doc = " DW_RLE_base_address"] BaseAddress { # [doc = " Base address."] address : Address , } , # [doc = " DW_RLE_offset_pair"] OffsetPair { # [doc = " Start of range relative to base address."] begin : u64 , # [doc = " End of range relative to base address."] end : u64 , } , # [doc = " DW_RLE_start_end"] StartEnd { # [doc = " Start of range."] begin : Address , # [doc = " End of range."] end : Address , } , # [doc = " DW_RLE_start_length"] StartLength { # [doc = " Start of range."] begin : Address , # [doc = " Length of range."] length : u64 , } , }
    };
}

Range!();