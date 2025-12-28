macro_rules! to_deadline {
    () => {
        # [inline] pub fn to_deadline (timeout : Duration) -> Option < Instant > { Instant :: now () . checked_add (timeout) }
    };
}

to_deadline!();