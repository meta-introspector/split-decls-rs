macro_rules! deps {
    () => {
        SYSTEMTIME!();
    };
}

macro_rules! TIME_ZONE_INFORMATION {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct TIME_ZONE_INFORMATION { pub Bias : i32 , pub StandardName : [u16 ; 32] , pub StandardDate : SYSTEMTIME , pub StandardBias : i32 , pub DaylightName : [u16 ; 32] , pub DaylightDate : SYSTEMTIME , pub DaylightBias : i32 , }
    };
}

TIME_ZONE_INFORMATION!()