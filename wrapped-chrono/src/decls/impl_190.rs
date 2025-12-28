macro_rules! deps {
    () => {
        TimeZone!();
        Duration!();
        DateTime!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < Tz : TimeZone > From < DateTime < Tz > > for SystemTime { fn from (dt : DateTime < Tz >) -> SystemTime { let sec = dt . timestamp () ; let nsec = dt . timestamp_subsec_nanos () ; if sec < 0 { UNIX_EPOCH - Duration :: new (- sec as u64 , 0) + Duration :: new (0 , nsec) } else { UNIX_EPOCH + Duration :: new (sec as u64 , nsec) } } }
    };
}

impl_190!();