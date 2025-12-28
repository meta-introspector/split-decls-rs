macro_rules! WallTime {
    () => {
        # [doc = " \"Monotonic clock\" with nanosecond precision (using [`std::time::Instant`])."] # [doc = ""] # [doc = " Can be obtained with `Counter::by_name(\"wall-time\")`."] pub struct WallTime { start : Instant , }
    };
}

WallTime!();