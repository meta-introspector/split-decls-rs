macro_rules! localtime {
    () => {
        # [cfg (feature = "local-time")] mod localtime { use std :: time :: SystemTime ; use jiff :: Zoned ; # [doc = " Return a string representing the current date and time as localtime."] # [doc = ""] # [doc = " Available with the `localtime` feature toggle."] pub fn format_now_datetime_seconds () -> String { Zoned :: now () . strftime ("%F %T %Z") . to_string () } # [doc = " Return a string representing the current time as localtime."] # [doc = ""] # [doc = " Available with the `localtime` feature toggle."] pub fn format_time_for_messages (time : SystemTime) -> String { Zoned :: try_from (time) . expect ("system time is always in range -9999-01-01..=9999-12-31") . strftime ("%T") . to_string () } }
    };
}

localtime!();