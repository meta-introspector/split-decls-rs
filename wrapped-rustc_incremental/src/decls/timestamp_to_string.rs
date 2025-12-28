macro_rules! timestamp_to_string {
    () => {
        fn timestamp_to_string (timestamp : SystemTime) -> BaseNString { let duration = timestamp . duration_since (UNIX_EPOCH) . unwrap () ; let micros : u64 = duration . as_micros () . try_into () . unwrap () ; micros . to_base_fixed_len (CASE_INSENSITIVE) }
    };
}

timestamp_to_string!()