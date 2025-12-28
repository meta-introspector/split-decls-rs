macro_rules! double_seconds_to_duration {
    () => {
        fn double_seconds_to_duration (seconds : f64) -> Duration { let whole_seconds = seconds . trunc () as u64 ; let nanos = seconds . fract () * 1_000_000_000f64 ; Duration :: new (whole_seconds , nanos as u32) }
    };
}

double_seconds_to_duration!()