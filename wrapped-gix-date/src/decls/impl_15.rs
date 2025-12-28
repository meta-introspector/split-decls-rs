macro_rules! deps {
    () => {
        Time!();
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Time { # [doc = " Produce a `Zoned` time for complex time computations and limitless formatting."] pub fn to_zoned (self) -> Result < jiff :: Zoned , jiff :: Error > { let offset = jiff :: tz :: Offset :: from_seconds (self . offset) ? ; Ok (jiff :: Timestamp :: from_second (self . seconds) ? . to_zoned (offset . to_time_zone ())) } }
    };
}

impl_15!()