macro_rules! from_bstring {
    () => {
        # [doc = " Similar to [`try_from_bstring()`], but will **panic** if there is ill-formed UTF-8 in the `input`."] pub fn from_bstring (input : impl Into < BString >) -> PathBuf { try_from_bstring (input) . expect ("well-formed UTF-8 on windows") }
    };
}

from_bstring!();