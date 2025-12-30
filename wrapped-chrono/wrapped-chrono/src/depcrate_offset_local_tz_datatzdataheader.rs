// Generated macro for TzDataHeader (struct)
macro_rules! Depcrate_offset_local_tz_dataTzDataHeader {
() => {
// Module: crate::offset::local::tz_data
// Provides: {"TzDataHeader"}
// Dependencies: {}
# [doc = " Header of the `tzdata` file."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] struct TzDataHeader { version : [u8 ; 5] , index_offset : u32 , data_offset : u32 , zonetab_offset : u32 , }
};
}
