macro_rules! deps {
    () => {
        FunctionMetrics!();
    };
}

macro_rules! record_function_entry {
    () => {
        deps!();
        pub fn record_function_entry (function_name : & str) { let mut metrics = METRICS . lock () . unwrap () ; let entry = metrics . entry (function_name . to_string ()) . or_insert_with (FunctionMetrics :: new) ; entry . start_time = Instant :: now () ; entry . call_count += 1 ; }
    };
}

record_function_entry!();