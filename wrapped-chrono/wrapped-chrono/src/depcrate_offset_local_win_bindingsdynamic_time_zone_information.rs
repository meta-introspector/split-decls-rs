// Generated macro for DYNAMIC_TIME_ZONE_INFORMATION (struct)
macro_rules! Depcrate_offset_local_win_bindingsDYNAMIC_TIME_ZONE_INFORMATION {
() => {
// Module: crate::offset::local::win_bindings
// Provides: {"DYNAMIC_TIME_ZONE_INFORMATION"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct DYNAMIC_TIME_ZONE_INFORMATION { pub Bias : i32 , pub StandardName : [u16 ; 32] , pub StandardDate : SYSTEMTIME , pub StandardBias : i32 , pub DaylightName : [u16 ; 32] , pub DaylightDate : SYSTEMTIME , pub DaylightBias : i32 , pub TimeZoneKeyName : [u16 ; 128] , pub DynamicDaylightTimeDisabled : bool , }
};
}
