macro_rules! deps {
    () => {
        OffsetInSeconds!();
        Time!();
        SecondsSinceUnixEpoch!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [doc = " Instantiation"] impl Time { # [doc = " Create a new instance from seconds and offset."] pub fn new (seconds : SecondsSinceUnixEpoch , offset : OffsetInSeconds) -> Self { Time { seconds , offset } } # [doc = " Return the current time without figuring out a timezone offset"] pub fn now_utc () -> Self { let seconds = jiff :: Timestamp :: now () . as_second () ; Self { seconds , offset : 0 } } # [doc = " Return the current local time, or `None` if the local time wasn't available."] pub fn now_local () -> Option < Self > { Some (Self :: now_local_or_utc ()) } # [doc = " Return the current local time, or the one at UTC if the local time wasn't available."] pub fn now_local_or_utc () -> Self { let zdt = jiff :: Zoned :: now () ; let seconds = zdt . timestamp () . as_second () ; let offset = zdt . offset () . seconds () ; Self { seconds , offset } } }
    };
}

impl_17!();