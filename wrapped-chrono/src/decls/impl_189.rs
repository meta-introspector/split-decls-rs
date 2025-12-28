macro_rules! deps {
    () => {
        Utc!();
        Local!();
        DateTime!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        # [cfg (feature = "clock")] impl From < SystemTime > for DateTime < Local > { fn from (t : SystemTime) -> DateTime < Local > { DateTime :: < Utc > :: from (t) . with_timezone (& Local) } }
    };
}

impl_189!();