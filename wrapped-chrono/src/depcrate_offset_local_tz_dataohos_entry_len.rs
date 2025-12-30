// Generated macro for OHOS_ENTRY_LEN (const)
macro_rules! Depcrate_offset_local_tz_dataOHOS_ENTRY_LEN {
() => {
// Module: crate::offset::local::tz_data
// Provides: {"OHOS_ENTRY_LEN"}
// Dependencies: {}
# [doc = " Ohos tzdata index entry size: `name + offset + length`"] # [cfg (any (test , target_env = "ohos"))] const OHOS_ENTRY_LEN : usize = TZ_NAME_LEN + 2 * size_of :: < u32 > () ;
};
}
