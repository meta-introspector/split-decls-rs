macro_rules! other_0 {
    () => {
        # [link (name = "profiler")] # [allow (non_snake_case)] extern "C" { fn ProfilerStart (fname : * const c_char) -> i32 ; fn ProfilerStop () ; }
    };
}

other_0!();