// Generated macro for fmt_duration (function)
macro_rules! Depcrate_progressfmt_duration {
() => {
// Module: crate::progress
// Provides: {"fmt_duration"}
// Dependencies: {}
# [doc = " Converts a [`std::time::Duration`] to a human readable format"] fn fmt_duration (duration : Duration) -> String { let as_secs = duration . as_secs_f64 () ; let as_min = (as_secs / 60.0) . floor () as usize ; let secs_portion : f64 = as_secs % 60.0 ; let min_portion : usize = ((as_secs - secs_portion) as usize / 60) % 60 ; let hr_portion : usize = ((as_min - min_portion) / 60) % 60 ; let mut output = String :: with_capacity (8) ; if hr_portion > 0 { write ! (& mut output , "{hr_portion}h ") . unwrap () ; } if min_portion > 0 { write ! (& mut output , "{min_portion}m ") . unwrap () ; } if as_secs > 60.0 && secs_portion != 0.0 { write ! (& mut output , "{:.0}s" , secs_portion . round ()) . unwrap () ; } else if secs_portion > 4.0 { write ! (& mut output , "{secs_portion:.1}s") . unwrap () ; } else if secs_portion > 1.0 { write ! (& mut output , "{secs_portion:.2}s") . unwrap () ; } else if secs_portion > 0.0 { write ! (& mut output , "{:.2}ms" , secs_portion * 1000.0) . unwrap () ; } output . trim () . to_string () }
};
}
