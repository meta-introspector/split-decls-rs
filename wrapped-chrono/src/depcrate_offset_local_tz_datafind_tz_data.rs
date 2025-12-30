// Generated macro for find_tz_data (function)
macro_rules! Depcrate_offset_local_tz_datafind_tz_data {
() => {
// Module: crate::offset::local::tz_data
// Provides: {"find_tz_data"}
// Dependencies: {}
# [doc = " Get timezone data from the `tzdata` file reader"] # [cfg (any (test , target_env = "ohos" , target_os = "android"))] fn find_tz_data < const ENTRY_LEN : usize > (mut reader : impl Read + Seek , tz_name : & [u8] ,) -> Result < Option < Vec < u8 > > > { let header = TzDataHeader :: new (& mut reader) ? ; let index = TzDataIndexes :: new :: < ENTRY_LEN > (& mut reader , & header) ? ; Ok (if let Some (entry) = index . find_timezone (tz_name) { Some (index . find_tzdata (reader , & header , entry) ?) } else { None }) }
};
}
