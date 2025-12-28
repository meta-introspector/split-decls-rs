macro_rules! deps {
    () => {
        Error!();
        Time!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl TryFrom < SystemTime > for Time { type Error = SystemTimeError ; fn try_from (s : SystemTime) -> Result < Self , SystemTimeError > { let d = s . duration_since (std :: time :: UNIX_EPOCH) ? ; Ok (Time { secs : d . as_secs () as u32 , nsecs : d . subsec_nanos () , }) } }
    };
}

impl_93!()