macro_rules! deps {
    () => {
        NaiveDateTime!();
        DateTime!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        # [doc = " The default value for a NaiveDateTime is 1st of January 1970 at 00:00:00."] # [doc = ""] # [doc = " Note that while this may look like the UNIX epoch, it is missing the"] # [doc = " time zone. The actual UNIX epoch cannot be expressed by this type,"] # [doc = " however it is available as [`DateTime::UNIX_EPOCH`]."] impl Default for NaiveDateTime { fn default () -> Self { DateTime :: UNIX_EPOCH . naive_local () } }
    };
}

impl_438!();