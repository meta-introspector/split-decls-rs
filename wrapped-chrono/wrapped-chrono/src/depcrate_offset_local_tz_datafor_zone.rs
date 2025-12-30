// Generated macro for for_zone (function)
macro_rules! Depcrate_offset_local_tz_datafor_zone {
() => {
// Module: crate::offset::local::tz_data
// Provides: {"for_zone"}
// Dependencies: {}
# [doc = " Get timezone data from the `tzdata` file of Android."] # [cfg (target_os = "android")] pub (crate) fn for_zone (tz_string : & str) -> Result < Option < Vec < u8 > > > { let mut file = open_android_tz_data_file () ? ; find_tz_data :: < ANDROID_ENTRY_LEN > (& mut file , tz_string . as_bytes ()) }
};
}
