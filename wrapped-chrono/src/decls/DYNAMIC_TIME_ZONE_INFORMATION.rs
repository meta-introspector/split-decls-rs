macro_rules! deps {
    () => {
        SYSTEMTIME!();
    };
}

macro_rules! DYNAMIC_TIME_ZONE_INFORMATION {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct DYNAMIC_TIME_ZONE_INFORMATION { pub Bias : i32 , pub StandardName : [u16 ; 32] , pub StandardDate : SYSTEMTIME , pub StandardBias : i32 , pub DaylightName : [u16 ; 32] , pub DaylightDate : SYSTEMTIME , pub DaylightBias : i32 , pub TimeZoneKeyName : [u16 ; 128] , pub DynamicDaylightTimeDisabled : bool , }
    };
}

DYNAMIC_TIME_ZONE_INFORMATION!();