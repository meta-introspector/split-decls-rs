// Generated macro for impl_757 (impl)
macro_rules! Depcrate_offset_local_tz_dataimpl_757 {
() => {
// Module: crate::offset::local::tz_data
// Provides: {"impl_757"}
// Dependencies: {}
impl TzDataHeader { # [doc = " Parse the header of the `tzdata` file."] fn new (mut data : impl Read) -> Result < Self > { let version = { let mut magic = [0 ; TZDATA_VERSION_LEN] ; data . read_exact (& mut magic) ? ; if ! magic . starts_with (b"tzdata") || magic [TZDATA_VERSION_LEN - 1] != 0 { return Err (Error :: new (ErrorKind :: Other , "invalid tzdata header magic")) ; } let mut version = [0 ; 5] ; version . copy_from_slice (& magic [6 .. 11]) ; version } ; let mut offset = [0 ; 4] ; data . read_exact (& mut offset) ? ; let index_offset = u32 :: from_be_bytes (offset) ; data . read_exact (& mut offset) ? ; let data_offset = u32 :: from_be_bytes (offset) ; data . read_exact (& mut offset) ? ; let zonetab_offset = u32 :: from_be_bytes (offset) ; Ok (Self { version , index_offset , data_offset , zonetab_offset }) } }
};
}
