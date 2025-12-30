// Generated macro for open_android_tz_data_file (function)
macro_rules! Depcrate_offset_local_tz_dataopen_android_tz_data_file {
() => {
// Module: crate::offset::local::tz_data
// Provides: {"open_android_tz_data_file"}
// Dependencies: {}
# [doc = " Open the `tzdata` file of Android from the environment variables."] # [cfg (target_os = "android")] fn open_android_tz_data_file () -> Result < File > { for (env_var , path) in [("ANDROID_DATA" , "/misc/zoneinfo") , ("ANDROID_ROOT" , "/usr/share/zoneinfo")] { if let Ok (env_value) = std :: env :: var (env_var) { if let Ok (file) = File :: open (format ! ("{}{}/tzdata" , env_value , path)) { return Ok (file) ; } } } Err (Error :: from (ErrorKind :: NotFound)) }
};
}
