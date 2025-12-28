macro_rules! utc {
    () => {
        # [cfg (not (feature = "local-time"))] mod utc { use std :: time :: SystemTime ; use super :: DATE_TIME_HMS ; # [doc = " Return a string representing the current date and time as UTC."] # [doc = ""] # [doc = " Available without the `localtime` feature toggle."] pub fn format_time_for_messages (time : SystemTime) -> String { let time = jiff :: Timestamp :: try_from (time) . expect ("reasonable system time") ; time . strftime ("%T") . to_string () } # [doc = " Return a string representing the current time as UTC."] # [doc = ""] # [doc = " Available without the `localtime` feature toggle."] pub fn format_now_datetime_seconds () -> String { jiff :: Timestamp :: now () . strftime ("%FT%T") . to_string () } }
    };
}

utc!();